# frozen_string_literal: true

RSpec.describe Oxidizer do
  it "has a version number" do
    expect(Oxidizer::VERSION).not_to be nil
  end

  it "does something useful" do
    expect(described_class.hello("World")).to eq("Hello from Rust, World!")
  end
end
