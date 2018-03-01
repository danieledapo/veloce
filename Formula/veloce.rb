class Veloce < Formula
  version 'v0.1.2'
  desc "Simple presto cli."
  homepage "https://github.com/d-dorazio/veloce"

  # shasum -a 256
  url "https://github.com/d-dorazio/veloce/releases/download/#{version}/veloce-#{version}-osx.tar.gz"
  sha256 "79cfd2556d850215b620383e27b1680cdcca43b6fcb9ceadfee2e05b95208aa2"

  def install
    bin.install "veloce"

    bash_completion.install "complete/veloce.bash"
    fish_completion.install "complete/veloce.fish"
    zsh_completion.install "complete/_veloce"
  end
end
