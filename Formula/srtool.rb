class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.6.0/srtool_macos_v0.6.0.tar.gz"
  sha256 "8574271d5ddd22eca42a53d7d49d3b935c78978411d43cc7bd4c7e95514412c7"
  version "0.6.0"

  def install
    bin.install "srtool"
  end
end
