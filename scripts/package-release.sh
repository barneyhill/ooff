#!/usr/bin/env bash
set -euo pipefail
: "${RELEASE_TARGET:?target triple required}"
version=$(cargo metadata --locked --no-deps --format-version 1 | python3 -c 'import json,sys; print(json.load(sys.stdin)["packages"][0]["version"])')
archive="oofft-v${version}-${RELEASE_TARGET}"
mkdir -p "dist/$archive"
cp target/release/oofft target/release/oofft-index target/release/oofft-ddg LICENSE LICENSES.md README.md "dist/$archive/"
cp docs/DDG.md "dist/$archive/DDG.md"
cp docs/SUMMARY.md "dist/$archive/SUMMARY.md"
mkdir -p "dist/$archive/docs" "dist/$archive/fixtures" "dist/$archive/scripts"
cp docs/USAGE.md docs/reference.md docs/DDG.md docs/SUMMARY.md "dist/$archive/docs/"
cp fixtures/queries.jsonl fixtures/reference.jsonl "dist/$archive/fixtures/"
cp scripts/relocate-index.py "dist/$archive/scripts/relocate-index.py"
cp src/energy/VIENNA_NOTICE "dist/$archive/VIENNA_NOTICE"
cp scripts/relocate-index.py "dist/$archive/relocate-index.py"
if [[ "$RELEASE_TARGET" == *apple-darwin ]]; then
    cp "$(brew --prefix libomp)/lib/libomp.dylib" "dist/$archive/"
    install_name_tool -id '@loader_path/libomp.dylib' "dist/$archive/libomp.dylib"
    for binary in oofft oofft-index oofft-ddg; do
        dependency=$(otool -L "dist/$archive/$binary" | awk '/libomp\.dylib/ {print $1}')
        if [[ -n "$dependency" ]]; then
            install_name_tool -change "$dependency" '@loader_path/libomp.dylib' "dist/$archive/$binary"
            dependency=$(otool -L "dist/$archive/$binary" | awk '/libomp\.dylib/ {print $1}')
            test "$dependency" = '@loader_path/libomp.dylib'
        fi
        codesign --force --sign - "dist/$archive/$binary"
    done
    codesign --force --sign - "dist/$archive/libomp.dylib"
    omp_prefix=$(brew --prefix libomp)
    license=$(find "$omp_prefix/" -maxdepth 4 -iname 'license*' -type f | head -n 1)
    test -n "$license"
    cp "$license" "dist/$archive/LICENSE-libomp.txt"
fi
"dist/$archive/oofft" --help > /dev/null
"dist/$archive/oofft-index" --help > /dev/null
"dist/$archive/oofft-ddg" --help > /dev/null
"dist/$archive/oofft" reference list --cache-dir "dist/$archive/reference-cache" > /dev/null
smoke_output=$(mktemp)
"dist/$archive/oofft" --queries "dist/$archive/fixtures/queries.jsonl" \
    --reference "dist/$archive/fixtures/reference.jsonl" > "$smoke_output"
python3 - "$smoke_output" <<'PYTHON'
import json, sys
with open(sys.argv[1]) as handle:
    rows = [json.loads(line) for line in handle]
assert rows[-1]['type'] == 'run_complete'
assert rows[1]['edit_distance_counts'] == {'0': 1, '1': 2, '2': 3, '3': 4}
PYTHON
tar -czf "dist/$archive.tar.gz" -C dist "$archive"
