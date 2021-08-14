extern crate reqwest;
extern crate scraper;
extern crate jstime_core;
extern crate pyo3;

use pyo3::prelude::*;
use jstime_core::JSTime;
use std::iter::FromIterator;
use scraper::{Html, Selector};
use pyo3::wrap_pyfunction;


fn eval_js(scope: &mut JSTime, str_script: &str) -> String {
    scope.run_script(&str_script, "jstime").expect("Error")
}

fn create_js_scope() -> JSTime {
    jstime_core::JSTime::new(
        jstime_core::Options::default()
    )
}

pub fn initialize_v8() -> bool {
    jstime_core::init(None);
    true
}

fn eval_js_mockup(scope: &mut JSTime, variable: &str) {
    let variable_parts = variable.split('.');
    let mut variable_chain = "".to_string();
    let mut variable_sep = "";
    let mut variable_prefix = "let ";

    for variable in variable_parts {
        if !variable_chain.is_empty() {
            variable_sep = ".";
            variable_prefix = "";
        }
        variable_chain = format!("{}{}{}", variable_chain, variable_sep, variable);

        let mut variable_with_prefix = format!(
            "{}{}", variable_prefix, variable_chain
        );

        if variable.contains('[') && variable.contains(']') {
            let mut before_bracket_variable_vec = Vec::new();
            let mut bracket_variable_vec = Vec::new();

            let mut bracket_started = false;
            let mut bracket_ended = false;

            for var_char in variable.chars() {
                if var_char == '[' && !bracket_ended {
                    bracket_started = true;
                }

                if bracket_started && !bracket_ended {
                    if var_char != '[' && var_char != ']' {
                        bracket_variable_vec.push(var_char);
                    }
                }

                if var_char == ']' && bracket_started {
                    bracket_ended = true;
                }

                if !bracket_started && !bracket_ended {
                    before_bracket_variable_vec.push(var_char);
                }
            }

            let bracket_variable = String::from_iter(bracket_variable_vec);

            variable_chain = variable_chain.replace(&format!("[{}]", &bracket_variable), "");
            variable_with_prefix = format!("{}{}", variable_prefix, variable_chain);

            eval_js(scope, &format!("{} = {{}}", variable_with_prefix));
            println!("Mockup: {} = {{}}", variable_with_prefix);

            eval_js(scope, &format!("{} = {{ {}: {{}} }}", variable_with_prefix, bracket_variable));
            println!("Mockup: {} = {{ {}: {{}} }}", variable_with_prefix, bracket_variable);
        } else {
            eval_js(scope, &format!("{} = {{}}", variable_with_prefix));
            println!("Mockup: {} = {{}}", format!("{}{}", variable_prefix, variable_chain));
        }
    }
}

fn get_html_by_url(url: &str) -> String {
    let client = reqwest::blocking::Client::new();
    let request = client.get(url);
    let response = request.send().unwrap();

    assert!(response.status().is_success());

    let html_text = response.text().unwrap();
    html_text
}

pub fn var_from_url(url: &str, variable: &str) -> String {
    let html_text = get_html_by_url(url);
    var_from_html(&html_text, variable)
}


pub fn var_from_html(html_text: &str, variable: &str) -> String {
    let html_dom = Html::parse_document(&html_text);
    let selector = Selector::parse("script").unwrap();
    let mut scope = create_js_scope();
    let mut variable_json = "".to_string();


    for node in html_dom.select(&selector) {
        if node.html().contains(&variable) {
            println!("Variable founded");

            let variables_raw = node.inner_html();
            let mut variable_raw = "".to_string();

            for v_r in variables_raw.split('\n') {
                if v_r.contains(&variable) {
                    variable_raw = v_r.parse().unwrap();
                    break;
                }
            }

            let variable_sanitized = &*variable_raw;

            if variable_sanitized.contains(" let ") {
                variable_raw = String::from(variable_sanitized.replacen("let ", "", 1));
            }
            else if variable_sanitized.contains(" const ") {
                variable_raw = String::from(variable_sanitized.replacen("const ", "", 1));
            }
            else if variable_sanitized.contains(" var ") {
                variable_raw = String::from(variable_sanitized.replacen("var ", "", 1));
            }


            eval_js_mockup(&mut scope, variable);
            println!("Mockuped");

            eval_js(&mut scope, &format!("{}", variable_raw));
            println!("Evaled");

            variable_json = eval_js(
                &mut scope, &format!("decodeURIComponent({})", &variable),
            );

            let variable_type = eval_js(
                &mut scope, &format!("typeof({})", &variable),
            );

            if variable_type == "object" {
                variable_json = eval_js(
                    &mut scope, &format!("JSON.stringify({})", &variable),
                );
            }

            break;
        }
    }

    if variable_json.is_empty() {
        println!("Variable not founded");
    }
    variable_json
}

#[pyfunction]
fn var_from_url_py(url: &str, variable: &str) -> PyResult<String> {
    Ok(var_from_url(&url, &variable))
}

#[pyfunction]
fn var_from_html_py(html_text: &str, variable: &str) -> PyResult<String> {
    Ok(var_from_html(&html_text, &variable))
}

#[pyfunction]
fn initialize_v8_py() -> PyResult<bool> {
    Ok(initialize_v8())
}

#[pymodule]
fn markup_parser(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(initialize_v8_py))?;
    m.add_wrapped(wrap_pyfunction!(var_from_html_py))?;
    m.add_wrapped(wrap_pyfunction!(var_from_url_py))?;

    Ok(())
}