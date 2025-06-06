import tzf
import pytest

from types import ModuleType

def test_it_gets_tz():
    assert tzf.get_tz(139.7744, 35.6812) == "Asia/Tokyo"

def test_it_gets_tzs():
    assert tzf.get_tzs(116.3883, 39.9289) == ["Asia/Shanghai"]

def test_it_has_a_data_version():
    assert tzf.data_version() == "2025b"
