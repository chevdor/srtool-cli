class Srtool < Formula
  desc "A command line utility to easily use the srtool docker image"
  homepage "https://github.com/chevdor/srtool-cli"
  url "https://github.com/chevdor/srtool-cli/releases/download/v0.8.0/srtool_macos_v0.8.0.tar.gz"
  sha256 "1a52a779f87ce3bd8ba25a4a91a491bcc29dfa6d5e79a284c6bd6193aa1e55ef"
  version "0.8.0"

  def install
    bin.install "srtool"
  end
end
