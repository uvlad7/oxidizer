# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("oxidizer/oxidizer") do |r|
  r.features = %w[ext-magnus]
end
