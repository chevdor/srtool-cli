class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.13.0/srtool_macos_v0.13.0.tar.gz"
  sha256 "5af88dda5bd213ab3a6a3fd23fc917bfba6e717b7ae238f517ebd1f2137b0b41"
  version "0.13.0"

  def install
    bin.install "srtool"
  end
end
