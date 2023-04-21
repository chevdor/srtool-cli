class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.11.0/srtool_macos_v0.11.0.tar.gz"
  sha256 "b30db9af89f00277a35be437f7f52e8f4754ee37c274d86fa1155cf79ed593bf"
  version "0.11.0"

  def install
    bin.install "srtool"
  end
end
