class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.12.0/srtool_macos_v0.12.0.tar.gz"
  sha256 "6817b8828787ec987e4844b5da390db0d0fca9fcf177d996b1466934bbb72798"
  version "0.12.0"

  def install
    bin.install "srtool"
  end
end
