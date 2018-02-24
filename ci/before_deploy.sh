#!/bin/sh

set -eux

# Generate artifacts for release
cargo clean --release || true
buildout="$(cargo build --release -vvv | grep 'OUT_DIR' | sed 's/OUT_DIR=\"\(.*\)\"$/\1/')"

# Create a temporary dir that contains our staging area.
# $tmpdir/$name is what eventually ends up as the deployed archive.
tmpdir="$(mktemp -d)"
name="veloce-${TRAVIS_TAG}-${TRAVIS_OS_NAME}"
staging="$tmpdir/$name"
mkdir -p "$staging"/complete

# The deployment directory is where the final archive will reside.
# This path is known by the .travis.yml configuration.
out_dir="$(pwd)/deployment"
mkdir -p "$out_dir"

cp "target/release/veloce" "$staging/veloce"

# Copy the licenses and README.
cp README.md "$staging/"

# Copy shell completion files.
cp "$buildout/veloce.bash" "$staging/complete/"
cp "$buildout/veloce.fish" "$staging/complete/"
cp "$buildout/_veloce" "$staging/complete/"
cp "$buildout/_veloce.ps1" "$staging/complete/"

(cd "$tmpdir" && tar czf "$out_dir/$name.tar.gz" "$name")
rm -rf "$tmpdir"
