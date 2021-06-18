class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.5.0/srtool_macos_v0.5.0.tar.gz"
  sha256 "b35a02d2e48dce84ec1bb268a2085ac944af85be743a93d8a9d1b67eb768d0b4"
  version "0.5.0"

  def install
    bin.install "srtool"
  end
end
