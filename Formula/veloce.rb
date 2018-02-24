class Veloce < Formula
  version 'v0.1'
  desc "Simple presto cli."
  homepage "https://github.com/d-dorazio/veloce"

  # shasum -a 256
  if OS.mac?
      url "https://github.com/d-dorazio/veloce/releases/download/#{version}/veloce-#{version}-osx.tar.gz"
      sha256 "2f12b891a57975c7639987c2dc7034ea5a7be384ed623a28acb219f376584a5b"
  elsif OS.linux?
      url "https://github.com/d-dorazio/veloce/releases/download/#{version}/veloce-#{version}-linux.tar.gz"
      sha256 "45e9d5996f049342ed0ec38501197533e2aafd00c059d0e5aa4c2ea4795bf9e3"
  end

  def install
    bin.install "veloce"

    bash_completion.install "complete/veloce.bash"
    fish_completion.install "complete/veloce.fish"
    zsh_completion.install "complete/_veloce"
  end
end
