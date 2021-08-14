from .markup_parser import *

initialized = False


def _init():
    global initialized
    if not initialized:
        initialized = True
        initialize_v8_py()


def var_from_url(url: str, variable: str):
    _init()
    return var_from_url_py(url, variable)


def var_from_html(html_text: str, variable: str):
    _init()
    return var_from_html_py(html_text, variable)



