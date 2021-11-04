class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.7.1/srtool_macos_v0.7.1.tar.gz"
  sha256 "92c26d6337efcb4718ae7cd580e0ee63cbefff812dcb7ea4571addaa23ec9020"
  version "0.7.1"

  def install
    bin.install "srtool"
  end
end
