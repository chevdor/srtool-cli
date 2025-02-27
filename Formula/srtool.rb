class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.13.0/srtool_macos_v0.13.0.tar.gz"
  sha256 "d5bfb37f99aacb9751c129e567052fb955484ea4df4e82beb9aa1199e89c6a67"
  version "0.13.0"

  def install
    bin.install "srtool"
  end
end
