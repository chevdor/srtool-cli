class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.7.0/srtool_macos_v0.7.0.tar.gz"
  sha256 "b37e488600129712e2d6024e79c0a13fab9de80a9d8b9e6ac794c967b96fbf8a"
  version "0.7.0"

  def install
    bin.install "srtool"
  end
end
