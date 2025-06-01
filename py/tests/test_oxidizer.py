import oxidizer
import pytest

def test_it_says_hello():
    assert oxidizer.hello("Python") == "Hello from Rust, Python!"

def test_it_works_with_excaptions():
    assert oxidizer.odd(1)
    with pytest.raises(Exception) as exc_info:
        oxidizer.odd(42)
    assert str(exc_info.value) == "42 is even"
