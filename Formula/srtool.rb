class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.13.2/srtool_macos_intel_v0.13.2.tar.gz"
  sha256 "b6d2e3dc396999bdd55d2aa0535f556f5171714eaca6653b504e8275e09f46b3"
  version "0.13.2"

  def install
    bin.install "srtool"
  end
end

