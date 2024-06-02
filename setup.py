from setuptools import setup
from setuptools_rust import RustExtension

if __name__ == "__main__":
    setup(
        rust_extensions=[RustExtension("markup_parser.markup_parser")],
    )
