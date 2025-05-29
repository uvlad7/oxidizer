# frozen_string_literal: true

require "toml-rb"

tag = ARGV[0]
gemspec = Gem::Specification.load(ARGV[1])
cargo = TomlRB.load_file(ARGV[2])

abort("Gem version does not match tag\n  v#{gemspec.version} != #{tag}") if "v#{gemspec.version}" != tag
cargo_version = cargo.dig("package", "version")
abort("Cargo version does not match tag\n  v#{cargo_version} != #{tag}") if "v#{cargo_version}" != tag
