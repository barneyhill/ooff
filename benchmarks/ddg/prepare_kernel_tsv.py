#!/usr/bin/env python3
"""Equivalent no-JSON input for the external C/Vienna kernel benchmark."""
import json, sys
from pathlib import Path
for name in sys.argv[1:]:
    path=Path(name)
    rows=json.loads(path.read_text())
    path.with_suffix('.tsv').write_text(''.join(f"{r['aso']}\t{r['target']}\t{r['cents']}\n" for r in rows))
