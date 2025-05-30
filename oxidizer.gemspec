# frozen_string_literal: true

require_relative "lib/oxidizer/version"

Gem::Specification.new do |spec|
  spec.name = "oxidizer-rb"
  spec.version = Oxidizer::VERSION
  spec.authors = ["uvlad7"]
  spec.email = ["uvlad7@gmail.com"]

  spec.summary = "TODO: Write a short summary, because RubyGems requires one."
  spec.description = "TODO: Write a longer description or delete this line."
  spec.homepage = "https://github.com/uvlad7/oxidizer"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 2.6"
  spec.required_rubygems_version = ">= 3.3.11"

  # spec.metadata["allowed_push_host"] = "TODO: Set to your gem server 'https://example.com'"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "#{spec.homepage}/tree/v#{spec.version}"
  spec.metadata["changelog_uri"] = "#{spec.homepage}/blob/main/CHANGELOG.md"
  spec.metadata["rubygems_mfa_required"] = "true"

  spec.files = [
    # .rb - to exclude .so files
    *Dir["ext/**/*"], *Dir["lib/**/*.rb"], *Dir["sig/**/*"],
    "Cargo.lock", "Cargo.toml"
  ].reject { |f| File.directory?(f) }
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/oxidizer/extconf.rb"]

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"
  spec.add_dependency "rb_sys", "~> 0.9.91"
  spec.add_development_dependency "irb"
  spec.add_development_dependency "rake", "~> 13.0"

  spec.add_development_dependency "rake-compiler"

  spec.add_development_dependency "rspec", "~> 3.0"

  spec.add_development_dependency "rubocop", "~> 1.21"

  # For version validation
  spec.add_development_dependency "toml-rb"

  # Hack for Ruby 3.4 compatibility
  spec.add_development_dependency "base64"

  spec.add_development_dependency "pry"
  spec.add_development_dependency "pry-byebug"
  spec.add_development_dependency "yard"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
