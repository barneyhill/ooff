#!/usr/bin/env bash
set -euo pipefail
: "${RELEASE_TARGET:?target triple required}"
version=$(cargo metadata --locked --no-deps --format-version 1 | python3 -c 'import json,sys; print(json.load(sys.stdin)["packages"][0]["version"])')
archive="ooff-v${version}-${RELEASE_TARGET}"
mkdir -p "dist/$archive"
cp target/release/ooff target/release/ooff-index LICENSE README.md "dist/$archive/"
if [[ "$RELEASE_TARGET" == *apple-darwin ]]; then
    cp "$(brew --prefix libomp)/lib/libomp.dylib" "dist/$archive/"
    install_name_tool -id '@loader_path/libomp.dylib' "dist/$archive/libomp.dylib"
    for binary in ooff ooff-index; do
        dependency=$(otool -L "dist/$archive/$binary" | awk '/libomp\.dylib/ {print $1}')
        test -n "$dependency"
        install_name_tool -change "$dependency" '@loader_path/libomp.dylib' "dist/$archive/$binary"
        codesign --force --sign - "dist/$archive/$binary"
    done
    codesign --force --sign - "dist/$archive/libomp.dylib"
    omp_prefix=$(brew --prefix libomp)
    license=$(find "$omp_prefix/" -maxdepth 4 -iname 'license*' -type f | head -n 1)
    test -n "$license"
    cp "$license" "dist/$archive/LICENSE-libomp.txt"
fi
"dist/$archive/ooff" --help > /dev/null
"dist/$archive/ooff-index" --help > /dev/null
tar -czf "dist/$archive.tar.gz" -C dist "$archive"
