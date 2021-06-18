class Srtool < Formula
  desc "A CLI utility to control the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/releases/download/v0.9.0/srtool-mac-v0.9.0.tar.gz"
  sha256 "a565e742be2e2b8e5e18dd0ac00f759a36236a9e2aa7aee6faf425502f3561ff"
  version "0.9.0"

  def install
    bin.install "srtool"
  end
end
