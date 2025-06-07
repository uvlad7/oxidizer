# frozen_string_literal: true

RSpec.describe Tzf do
  it "has a version number" do
    expect(Tzf::VERSION).not_to be nil
  end

  it "gets tz" do
    expect(described_class.get_tz(139.7744, 35.6812)).to eq("Asia/Tokyo")
  end

  it "gets tzs" do
    expect(described_class.get_tzs(116.3883, 39.9289)).to eq(["Asia/Shanghai"])
  end

  it "has a data version" do
    expect(described_class.data_version).to eq("2025b")
  end
end
