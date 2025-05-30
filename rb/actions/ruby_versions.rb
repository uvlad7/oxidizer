# frozen_string_literal: true

require "net/http"
require "json"

RUBY_VERSIONS_URL = "https://endoflife.date/api/ruby.json"

gemspec = Gem::Specification.load(ARGV[0])

puts(JSON.parse(Net::HTTP.get(URI(RUBY_VERSIONS_URL))).map do |ver|
  ver["cycle"] if gemspec.required_ruby_version.satisfied_by?(Gem::Version.new(ver["cycle"]))
end.compact.to_json)
