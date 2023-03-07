class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.10.0/srtool_macos_v0.10.0.tar.gz"
  sha256 "87e0ac31ee3ddfab13c5450be817fec3a1f8136b9f6e0ed36cb69222fe458266"
  version "0.10.0"

  def install
    bin.install "srtool"
  end
end
