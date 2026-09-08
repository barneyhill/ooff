#!/usr/bin/env python3
"""Verify site annotations against direct ViennaRNA on actual discovery intervals."""
import datetime
import hashlib
import json
import re
import subprocess
import time
from pathlib import Path

root = Path('benchmarks/ddg/iterations') / (datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S.%f') + '-site-annotation-oracle')
root.mkdir(parents=True)
record = dict(label='site-annotation-oracle', threads=4, attempts=[], complete=False)
rna = '/home/barneyh/.local/bin/RNAduplex'
binary = 'target/release/oofft-ddg'
queries = Path('fixtures/queries.jsonl')
reference = root/'reference.jsonl'
source_reference = Path('fixtures/reference.jsonl')
first_aso = json.loads(queries.read_text().splitlines()[0])['sequence']
target = first_aso.translate(str.maketrans('ACGT', 'TGCA'))[::-1]
extra = []
for insertion in [1, 2, 3]:
    sequence = target[:8] + 'A'*insertion + target[8:]
    extra.append(dict(id=f'insertion{insertion}', sequence=sequence, genes=['other'],
                      transcripts=[f'inserted-transcript{insertion}'], contig='fixture', strand='+',
                      blocks=[dict(start=0, end=len(sequence))]))
reference.write_text(source_reference.read_text()+''.join(json.dumps(r)+'\n' for r in extra))
record['sha256'] = {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in [queries, reference, source_reference, Path(binary), Path(rna)]}

def execute(name, command, stdin=None):
    started = time.perf_counter()
    result = subprocess.run(command, input=stdin, text=True, capture_output=True, timeout=600)
    elapsed = time.perf_counter() - started
    record['attempts'].append(dict(engine=name, repetition=0, command=command, seconds=elapsed, returncode=result.returncode))
    (root/(name+'.stdout')).write_text(result.stdout)
    (root/(name+'.stderr')).write_text(result.stderr)
    assert result.returncode == 0, result.stderr
    return result.stdout

try:
    report = execute('discovery', ['target/release/oofft', 'report', '--queries', str(queries), '--reference', str(reference),
                                  '--policy', 'other-gene', '--reference-release', 'fixture-v1', '--scope', 'synthetic',
                                  '--biotype-policy', 'all-fixture-records', '-k', '3'])
    rows = [json.loads(line) for line in report.splitlines()]
    qs = {q['id']: q for q in map(json.loads, queries.read_text().splitlines())}
    refs = {r['id']: r['sequence'].upper().replace('T', 'U') for r in map(json.loads, reference.read_text().splitlines())}
    pairs = []
    expected_sites = []
    for row in rows:
        if row['type'] != 'site':
            continue
        aso = qs[row['query_id']]['sequence'].upper().replace('T', 'U')
        target = aso.translate(str.maketrans('ACGU', 'UGCA'))[::-1]
        site = refs[row['record_id']][row['alignment']['start']:row['alignment']['end']]
        pairs.extend([(aso, target), (aso, site)])
        expected_sites.append(row)
    stdin = ''.join(a+'\n'+t+'\n' for a, t in pairs)
    (root/'viennarna.input').write_text(stdin)
    oracle = execute('viennarna', [rna, '--noconv'], stdin)
    energies = [float(re.search(r'\(\s*(-?\d+\.\d+)\s*\)\s*$', line)[1]) for line in oracle.splitlines() if '&' in line]
    assert len(energies) == len(pairs)
    actual = execute('ooff', [binary, '--sites', str(root/'discovery.stdout'), '--queries', str(queries),
                             '--reference', str(reference), '--rnaduplex', rna, '--threads', '4', '--batch-size', '7'])
    actual_rows = [json.loads(line) for line in actual.splitlines()]
    assert len(actual_rows) == len(rows)
    index = 0
    for expected, annotated in zip(rows, actual_rows):
        annotation = annotated.pop('energy_annotation', None)
        if expected['type'] == 'site':
            on, off = energies[2*index:2*index+2]
            assert abs(annotated.pop('ddg') - (off-on)) <= 1e-12
            assert annotation['dg_target'] == on and annotation['dg_other'] == off
            assert annotation['scope'] == 'site'
            assert annotation['scored_record_interval'] == [expected['alignment']['start'], expected['alignment']['end']]
            index += 1
        assert annotated == expected
    record.update(complete=True, all_fields_match=True, annotated_sites=index,
                  target_lengths=sorted({len(t) for _, t in pairs}))
    assert record['target_lengths'] == list(range(17, 24))
    # Whole-record annotations must retain the actual OligoAI convention,
    # including exact-match shortcuts, independently of the site calculation.
    oracle_queries = root/'oligoai-queries.jsonl'
    oracle_queries.write_text(''.join(json.dumps(dict(id=qid, aso=q['sequence'],
        target=q['sequence'].translate(str.maketrans('ACGT', 'TGCA'))[::-1]))+'\n' for qid, q in qs.items()))
    whole_expected = {}
    for n, rid in enumerate(sorted({r['record_id'] for r in expected_sites})):
        target_file = root/f'record-{n}.fa'
        target_file.write_text('>offtarget\n'+refs[rid]+'\n')
        text = execute(f'oligoai-record-{n}', ['/home/barneyh/.bun/bin/bun', 'benchmarks/ddg/oligoai_oracle.ts',
            '/home/barneyh/oligoai-v2/src/specificity.ts', str(oracle_queries.resolve()), str(target_file.resolve()), '20'])
        whole_expected[rid] = {v['id']: v for v in map(json.loads, text.splitlines())}
    whole = execute('ooff-whole-transcript', [binary, '--sites', str(root/'discovery.stdout'),
        '--queries', str(queries), '--reference', str(reference), '--rnaduplex', rna,
        '--rnaplex', '/home/barneyh/.local/bin/RNAplex', '--whole-transcript', '--threads', '4'])
    whole_rows = [json.loads(line) for line in whole.splitlines()]
    assert len(whole_rows) == len(rows)
    for original, annotated in zip(rows, whole_rows):
        annotation = annotated.pop('energy_annotation', None)
        if original['type'] == 'site':
            expected = whole_expected[original['record_id']][original['query_id']]
            if expected['ddg'] is not None:
                expected['ddg'] = -expected['ddg']  # OligoAI v2 uses the opposite sign.
            actual_ddg = annotated.pop('ddg')
            assert actual_ddg == expected['ddg'] or (actual_ddg is not None and expected['ddg'] is not None and abs(actual_ddg-expected['ddg']) <= 1e-12)
            for key in ['dg_target', 'dg_other']:
                assert annotation[key] == expected[key]
            assert annotation['whole_transcript_best_position_1based'] == expected['dg_other_position']
            assert annotation['whole_transcript_exact_match_shortcut'] == expected['exact_off_target_match']
        assert annotated == original
    record['whole_transcript_matches_oligoai'] = True
    print(f'PASS: {index} distinct discovered intervals; all energies, coordinates and report metadata match')
except Exception as error:
    record['complete'] = False
    record['error'] = str(error)
    raise
finally:
    (root/'result.json').write_text(json.dumps(record, indent=2)+'\n')
print(root)
