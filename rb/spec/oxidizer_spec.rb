# frozen_string_literal: true

RSpec.describe Oxidizer do
  it "has a version number" do
    expect(Oxidizer::VERSION).not_to be nil
  end

  it "says hello" do
    expect(described_class.hello("Ruby")).to eq("Hello from Rust, Ruby!")
  end

  it "works with exceptions" do
    expect(described_class.odd(1)).to eq(true)
    expect { described_class.odd(42) }.to raise_error(RuntimeError, "42 is even")
  end

  it "creates submodules" do
    expect(described_class::SnakeCase).to be_a(Module)
    expect(described_class::SnakeCase.repr(described_class::SnakeCase)).to eq('Oxidizer::SnakeCase')
    expect(described_class::CamelCase).to be_a(Module)
    expect(described_class::SnakeCase.repr(described_class::CamelCase)).to eq('Oxidizer::CamelCase')
  end
end
