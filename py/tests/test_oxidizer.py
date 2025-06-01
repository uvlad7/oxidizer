import oxidizer
import pytest

from types import ModuleType

def test_it_says_hello():
    assert oxidizer.hello("Python") == "Hello from Rust, Python!"

def test_it_works_with_exceptions():
    assert oxidizer.odd(1)
    with pytest.raises(Exception) as exc_info:
        oxidizer.odd(42)
    assert str(exc_info.value) == "42 is even"

def test_it_creates_submodules():
    assert isinstance(oxidizer.snake_case, ModuleType)
    assert oxidizer.snake_case.inspect(oxidizer.snake_case) == "<module 'snake_case'>"
    assert isinstance(oxidizer.camel_case, ModuleType)
    assert oxidizer.snake_case.inspect(oxidizer.camel_case) == "<module 'camel_case'>"
