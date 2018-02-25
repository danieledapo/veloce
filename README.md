# veloce [![Build Status](https://travis-ci.org/d-dorazio/veloce.svg?branch=master)](https://travis-ci.org/d-dorazio/veloce)
Simple Presto client written in rust.

# How to install

Binaries are available on [Github](https://github.com/d-dorazio/veloce/releases).

On OSX you can use homebrew
```
brew tap d-dorazio/veloce https://github.com/d-dorazio/veloce.git
brew info veloce
brew install veloce
```

To install the development version you can use `cargo`
```
cargo install --git https://github.com/d-dorazio/veloce
```

# Features

- custom pager configuration
- multiline query support
- primitive query auto completer
- query history
- shell comletions
