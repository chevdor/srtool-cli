class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.13.1/srtool_macos_intel_v0.13.1.tar.gz"
  sha256 "7e6c9fddeea7b6842c9ebf4f7cdeb679966a19071beaa5ee74f5e06fab50fd0a"
  version "0.13.1"

  def install
    bin.install "srtool"
  end
end

