# Markup Parser
#### Extract JS variables content from HTML markup
[![Codecov](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://travis-ci.com/bigpe/markup-parser.svg?token=9PVJVxRxQ4uXFdey5v3k&branch=master)](https://travis-ci.com/bigpe/markup-parser)
[![Versions](https://img.shields.io/pypi/pyversions/markup-parser.svg)](https://pypi.org/project/markup-parser/)
[![Release](https://img.shields.io/github/release/bigpe/markup-parser.svg)](https://github.com/bigpe/markup-parser/releases)

### ENG
[RU](#ru)

## Installation

```shell
pip install markup-parser
```

## Usage

### Parse variable from html text:

Test content
```html
<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Test</title>
    </head>
    <body>
    
    </body>
    <script>
        let test_variable = 'You found me';
    </script>
</html>
```

```python
import requests
from markup_parser import var_from_html

# Fetch html content by get request
html_text = requests.get('http://test.html').text
var_from_html(html_text, 'test_variable')

# Parse from file
var_from_html(open('test.html').read(), 'test_variable')

# >>> You found me
```

### Parse variable from url:

_* This variant not recommended if you need to specify custom headers, this feature may be added in future release_

Test content
```html
<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Test</title>
    </head>
    <body>
    
    </body>
    <script>
        let test_variable = 'You found me again!';
    </script>
</html>
```

```python
from markup_parser import var_from_url

var_from_url('http://test.page', 'test_variable')

# >>> You found me again!
```

### RU
[ENG](#eng)

## Установка

```shell
pip markup-parser
```

## Примеры использования

### Получить значение переменной из html разметки:

Пример html страницы
```html
<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Test</title>
    </head>
    <body>
    
    </body>
    <script>
        let test_variable = 'You found me';
    </script>
</html>
```

```python
import requests
from markup_parser import var_from_html

# Получить html разметку с помощью get запроса
html_text = requests.get('http://test.html').text
var_from_html(html_text, 'test_variable')

# Прочитать из файла
var_from_html(open('test.html').read(), 'test_variable')

# >>> You found me
```

### Получить значение переменной по ссылке:

_* На данный момент этот вариант не рекомендуется если вам необходимы специфичные заголовки, эта функция может появтися в будущих релизах_

Пример html страницы
```html
<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Test</title>
    </head>
    <body>
    
    </body>
    <script>
        let test_variable = 'You found me again!';
    </script>
</html>
```

```python
from markup_parser import var_from_url

var_from_url('http://test.page', 'test_variable')

# >>> You found me again!
```