class Veloce < Formula
  version 'v0.1.1'
  desc "Simple presto cli."
  homepage "https://github.com/d-dorazio/veloce"

  # shasum -a 256
  url "https://github.com/d-dorazio/veloce/releases/download/#{version}/veloce-#{version}-osx.tar.gz"
  sha256 "8d1940cae299a91ba773a323f46ac699c157abd50fc6cd009e5765cf9dc34aa9"

  def install
    bin.install "veloce"

    bash_completion.install "complete/veloce.bash"
    fish_completion.install "complete/veloce.fish"
    zsh_completion.install "complete/_veloce"
  end
end
