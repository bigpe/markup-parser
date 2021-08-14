from pathlib import Path
from unittest import TestCase

from markup_parser import var_from_html

html_text = open(Path(__file__).resolve().parent.joinpath('test_with_prefix.html')).read()
success = 'success'


class MarkupParserTestCase(TestCase):

    def test_find_let_variable(self):
        global success
        global html_text

        result = var_from_html(html_text, "test_let")
        self.assertEqual(result, success)

    def test_find_const_variable(self):
        global success
        global html_text

        result = var_from_html(html_text, "test_const")
        self.assertEqual(result, success)

    def test_find_var_variable(self):
        global success
        global html_text

        result = var_from_html(html_text, "test_var")
        self.assertEqual(result, success)

