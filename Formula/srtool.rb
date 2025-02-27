class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.13.1/srtool_macos_intel_v0.13.1.tar.gz"
  sha256 "84010f005b683ed7dde68ffec8effc8b39c1f30f7a9ae47c3f67564f1ef8527e"
  version "0.13.1"

  def install
    bin.install "srtool"
  end
end

