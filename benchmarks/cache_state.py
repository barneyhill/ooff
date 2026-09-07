"""Evict only benchmark input files; measure remaining residency with mincore.

This changes the OS page cache, never file contents or unrelated system caches.
Run only when no benchmark process is using these immutable input files.
"""
import ctypes
import mmap
import os
from pathlib import Path


def evict(paths):
    libc = ctypes.CDLL(None, use_errno=True)
    libc.mincore.argtypes = [ctypes.c_void_p, ctypes.c_size_t, ctypes.c_void_p]
    libc.mincore.restype = ctypes.c_int
    results = []
    for path in sorted(set(map(Path, paths))):
        with path.open('rb') as file:
            size = os.fstat(file.fileno()).st_size
            os.posix_fadvise(file.fileno(), 0, 0, os.POSIX_FADV_DONTNEED)
            pages = (size + mmap.PAGESIZE - 1) // mmap.PAGESIZE
            resident = 0
            if size:
                with mmap.mmap(file.fileno(), 0, access=mmap.ACCESS_COPY) as mapping:
                    address = ctypes.c_char.from_buffer(mapping)
                    vector = (ctypes.c_ubyte * pages)()
                    result = libc.mincore(ctypes.addressof(address), size, vector)
                    del address
                    if result != 0:
                        raise OSError(ctypes.get_errno(), 'mincore failed')
                    resident = sum(value & 1 for value in vector)
            results.append(dict(path=str(path), bytes=size, pages=pages,
                                resident_pages_after_eviction=resident))
    return results
