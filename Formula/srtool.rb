class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.9.0/srtool_macos_v0.9.0.tar.gz"
  sha256 "8ac0980b0662283316b4fd06adbfba9c6e8695c0c5c6f6b89163092e650b271f"
  version "0.9.0"

  def install
    bin.install "srtool"
  end
end
