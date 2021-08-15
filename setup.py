from setuptools import setup
from setuptools_rust import RustExtension

setup(
    rust_extensions=[RustExtension("markup_parser.markup_parser")],
)
