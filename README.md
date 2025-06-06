# Template project

This is a template project that allows to create both a Ruby gem and a Python library implemented in Rust.
For the Ruby part, Bundler was used to initialize the project:
```bash
bundle gem oxidizer-rb --ext=rust --mit --test=rspec --ci=github --linter=rubocop
```
For the Python part, Maturin was used:
```bash
python -m venv env
pip install maturin
maturin init --name oxidizer-py --bindings pyo3
```

Mise is used to manage tool versions. If you don’t have it installed,
you can manually install the tools specified in these files: 
- [`mise.toml`](./mise.toml)
- [`rb/mise.toml`](./rb/mise.toml)
- [`py/mise.toml`](./py/mise.toml)

##### Ruby Gem
To test the Ruby gem, navigate to the `rb` directory and run:
```shell
bin/setup
bin/console
```

##### Python Library
For the Python library, go to the `py` directory and execute:
```shell
pip install -e '.[dev]'
ipython
```

### Example of `tzfpy` migration

```diff
-use pyo3::prelude::*;
+use oxide::{oxy_function, wrap_oxyfunction};
+use oxide::{oxy_init, OxyModule, OxyResult, Python};
 use lazy_static::lazy_static;
 use tzf_rs::DefaultFinder;
 
@@ -6,31 +7,31 @@ lazy_static! {
     static ref FINDER: DefaultFinder = DefaultFinder::default();
 }
 
-#[pyfunction]
-pub fn get_tz(lng: f64, lat: f64) -> PyResult<String> {
+#[oxy_function]
+pub fn get_tz(lng: f64, lat: f64) -> OxyResult<String> {
     Ok(FINDER.get_tz_name(lng, lat).to_string())
 }
 
-#[pyfunction]
-pub fn get_tzs(_py: Python, lng: f64, lat: f64) -> PyResult<Vec<&str>> {
+#[oxy_function]
+pub fn get_tzs(_py: Python, lng: f64, lat: f64) -> OxyResult<Vec<&str>> {
     Ok(FINDER.get_tz_names(lng, lat))
 }
 
-#[pyfunction]
-pub fn timezonenames(_py: Python) -> PyResult<Vec<&str>> {
+#[oxy_function]
+pub fn timezonenames(_py: Python) -> OxyResult<Vec<&str>> {
     return Ok(FINDER.timezonenames());
 }
 
-#[pyfunction]
-pub fn data_version(_py: Python) -> PyResult<String> {
+#[oxy_function]
+pub fn data_version(_py: Python) -> OxyResult<String> {
     return Ok(FINDER.data_version().to_string());
 }
 
-#[pymodule]
-fn tzfpy(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
-    m.add_function(wrap_pyfunction!(get_tz, m)?)?;
-    m.add_function(wrap_pyfunction!(get_tzs, m)?)?;
-    m.add_function(wrap_pyfunction!(timezonenames, m)?)?;
-    m.add_function(wrap_pyfunction!(data_version, m)?)?;
+#[oxy_init]
+fn tzfpy(m: &OxyModule) -> OxyResult<()> {
+    m.add_function(wrap_oxyfunction!(get_tz, m)?)?;
+    m.add_function(wrap_oxyfunction!(get_tzs, m)?)?;
+    m.add_function(wrap_oxyfunction!(timezonenames, m)?)?;
+    m.add_function(wrap_oxyfunction!(data_version, m)?)?;
     Ok(())
 }
```

The rest of the ReadMe is a template.

[![Gem Version](https://badge.fury.io/rb/tzfrb.svg)](https://badge.fury.io/rb/tzfrb)
![Gem](https://img.shields.io/gem/dt/tzfrb?style=plastic)
![Gem](https://img.shields.io/gem/dtv/tzfrb?style=plastic)
[![Tests](https://github.com/uvlad7/tzfrb/actions/workflows/main.yml/badge.svg)](https://github.com/uvlad7/tzfrb/actions/workflows/main.yml)
[![Docs](https://github.com/uvlad7/tzfrb/actions/workflows/docs.yml/badge.svg)](https://github.com/uvlad7/tzfrb/actions/workflows/docs.yml)
[![Release](https://github.com/uvlad7/tzfrb/actions/workflows/release.yml/badge.svg)](https://github.com/uvlad7/tzfrb/actions/workflows/release.yml)

# Tzf

TODO: Delete this and the text below, and describe your gem

Welcome to your new gem! In this directory, you'll find the files you need to be able to package up your Ruby library into a gem. Put your Ruby code in the file `lib/tzf`. To experiment with that code, run `bin/console` for an interactive prompt.

## Installation

TODO: Replace `UPDATE_WITH_YOUR_GEM_NAME_IMMEDIATELY_AFTER_RELEASE_TO_RUBYGEMS_ORG` with your gem name right after releasing it to RubyGems.org. Please do not do it earlier due to security reasons. Alternatively, replace this section with instructions to install your gem from git if you don't plan to release to RubyGems.org.

Install the gem and add to the application's Gemfile by executing:

```bash
bundle add UPDATE_WITH_YOUR_GEM_NAME_IMMEDIATELY_AFTER_RELEASE_TO_RUBYGEMS_ORG
```

If bundler is not being used to manage dependencies, install the gem by executing:

```bash
gem install UPDATE_WITH_YOUR_GEM_NAME_IMMEDIATELY_AFTER_RELEASE_TO_RUBYGEMS_ORG
```

## Usage

TODO: Write usage instructions here

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/[USERNAME]/tzfrb.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
