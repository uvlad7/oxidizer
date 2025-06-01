import pytest
import maturin_import_hook
from maturin_import_hook.settings import MaturinSettings

# install the import hook with default settings.
# this call must be before any imports that you want the hook to be active for.

maturin_import_hook.install(
    settings=MaturinSettings(
        features=["pyo3/extension-module", "ext-pyo3"]
    )
)
