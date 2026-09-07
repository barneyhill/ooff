"""Exact unordered tuple comparison, with bounded-memory partitioning.

CRC32 only routes rows to buckets; full rows are compared for equality, so hash
collisions cannot hide differences. All raw and bucket artifacts are retained.
"""
from collections import Counter
from pathlib import Path
import zlib


def compare(left, right, folder, threshold=64 * 1024 * 1024):
    left, right, folder = Path(left), Path(right), Path(folder)
    if max(left.stat().st_size, right.stat().st_size) <= threshold:
        with left.open('rb') as a, right.open('rb') as b:
            aa, bb = Counter(a), Counter(b)
        return dict(equal=aa == bb, unique=all(n == 1 for n in aa.values())
                    and all(n == 1 for n in bb.values()),
                    left_rows=sum(aa.values()), right_rows=sum(bb.values()), buckets=0)
    folder.mkdir(exist_ok=False)
    buckets = 128
    for name, path in [('left', left), ('right', right)]:
        outputs = [(folder / f'{name}-{i:03}.rows').open('xb') for i in range(buckets)]
        try:
            with path.open('rb') as source:
                for line in source:
                    outputs[zlib.crc32(line) % buckets].write(line)
        finally:
            for out in outputs:
                out.close()
    equal, unique, left_rows, right_rows = True, True, 0, 0
    for i in range(buckets):
        with (folder / f'left-{i:03}.rows').open('rb') as a, (folder / f'right-{i:03}.rows').open('rb') as b:
            aa, bb = Counter(a), Counter(b)
        equal &= aa == bb
        unique &= all(n == 1 for n in aa.values()) and all(n == 1 for n in bb.values())
        left_rows += sum(aa.values())
        right_rows += sum(bb.values())
    return dict(equal=equal, unique=unique, left_rows=left_rows,
                right_rows=right_rows, buckets=buckets)
