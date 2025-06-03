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

The rest of the ReadMe is a template.

[![Gem Version](https://badge.fury.io/rb/oxidizer-rb.svg)](https://badge.fury.io/rb/oxidizer-rb)
![Gem](https://img.shields.io/gem/dt/oxidizer-rb?style=plastic)
![Gem](https://img.shields.io/gem/dtv/oxidizer-rb?style=plastic)
[![Tests](https://github.com/uvlad7/oxidizer-rb/actions/workflows/main.yml/badge.svg)](https://github.com/uvlad7/oxidizer-rb/actions/workflows/main.yml)
[![Docs](https://github.com/uvlad7/oxidizer-rb/actions/workflows/docs.yml/badge.svg)](https://github.com/uvlad7/oxidizer-rb/actions/workflows/docs.yml)
[![Release](https://github.com/uvlad7/oxidizer-rb/actions/workflows/release.yml/badge.svg)](https://github.com/uvlad7/oxidizer-rb/actions/workflows/release.yml)

# Oxidizer

TODO: Delete this and the text below, and describe your gem

Welcome to your new gem! In this directory, you'll find the files you need to be able to package up your Ruby library into a gem. Put your Ruby code in the file `lib/oxidizer`. To experiment with that code, run `bin/console` for an interactive prompt.

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

Bug reports and pull requests are welcome on GitHub at https://github.com/[USERNAME]/oxidizer.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
