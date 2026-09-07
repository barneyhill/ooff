#!/usr/bin/env python3
"""Build documented RNA-sense gene-body and mature-transcript fixtures on EC2."""
import argparse
import collections
import gzip
import hashlib
import json
from pathlib import Path
import re
import time


def allowed(biotype):
    return "pseudogene" not in biotype or biotype.startswith(("transcribed_", "translated_"))


def fasta(path):
    with gzip.open(path, "rt") as handle:
        name, parts = None, []
        for line in handle:
            if line.startswith(">"):
                if name is not None:
                    yield name, "".join(parts)
                name, parts = line[1:].split()[0], []
            else:
                parts.append(line.strip())
        if name is not None:
            yield name, "".join(parts)


def rc(seq):
    return seq.translate(str.maketrans("ACGTacgt", "TGCAtgca"))[::-1]


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--raw", type=Path, default=Path("data/raw"))
    ap.add_argument("--out", type=Path, default=Path("data/reference-v1"))
    args = ap.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    start = time.monotonic()
    genes, transcripts = {}, {}
    biotypes = collections.Counter()
    with gzip.open(args.raw / "Homo_sapiens.GRCh38.110.gtf.gz", "rt") as handle:
        for line in handle:
            if line.startswith("#"):
                continue
            c, _, kind, a, b, _, strand, _, attributes = line.rstrip().split("\t")
            if kind not in ("gene", "transcript", "exon"):
                continue
            fields = dict(re.findall(r'(\w+) "([^"]*)";', attributes))
            gene = fields["gene_id"]
            if kind == "gene":
                bt = fields["gene_biotype"]
                biotypes[bt] += 1
                if allowed(bt):
                    genes[gene] = dict(id=gene, contig=c, start=int(a)-1, end=int(b), strand=strand, biotype=bt, name=fields.get("gene_name"))
            else:
                tx = transcripts.setdefault(fields["transcript_id"], dict(gene=gene, contig=c, strand=strand, exons=[]))
                if kind == "exon":
                    tx["exons"].append((int(a)-1, int(b)))
    by_contig = collections.defaultdict(list)
    for g in genes.values():
        by_contig[g["contig"]].append(("gene_body", g["id"], g, [(g["start"],g["end"])]))
    for tid, tx in transcripts.items():
        if tx["gene"] not in genes:
            continue
        blocks = sorted(set(tx["exons"]), reverse=tx["strand"] == "-")
        if not blocks:
            raise ValueError(f"Transcript without exons: {tid}")
        by_contig[tx["contig"]].append(("mature", tid, genes[tx["gene"]], blocks))
    counts = collections.Counter()
    seen = set()
    digest = hashlib.sha256()
    with (args.out / "reference.fa").open("x") as fa, (args.out / "records.jsonl").open("x") as meta, (args.out / "bounded-10mb.fa").open("x") as small:
        for contig, sequence in fasta(args.raw / "Homo_sapiens.GRCh38.dna_sm.primary_assembly.fa.gz"):
            seen.add(contig)
            for kind, rid, gene, blocks in by_contig[contig]:
                if any(a < 0 or b > len(sequence) or a >= b for a,b in blocks):
                    raise ValueError(f"Invalid genomic blocks: {rid}")
                pieces = [sequence[a:b] for a,b in blocks]
                if gene["strand"] == "-":
                    pieces = [rc(p) for p in pieces]
                seq = "".join(pieces)
                identifier = f"{kind}:{rid}"
                record = dict(id=identifier,genes=[gene["id"]], transcripts=[rid] if kind == "mature" else [], contig=contig,strand=gene["strand"],blocks=[dict(start=a,end=b) for a,b in blocks],kind=kind,biotype=gene["biotype"],length=len(seq))
                serialized = f'>{identifier}|{gene["id"]}\n{seq}\n'
                fa.write(serialized)
                digest.update(serialized.encode())
                meta.write(json.dumps(record)+"\n")
                # Whole records only, all classes remain in full reference.
                if counts["bounded_bases"] + len(seq) <= 10_000_000:
                    small.write(serialized)
                    counts["bounded_bases"] += len(seq)
                counts[kind+"_records"] += 1
                counts["bases"] += len(seq)
                counts["unknown_bases"] += sum(ch.upper() not in "ACGT" for ch in seq)
                counts["softmasked_bases"] += sum(ch.islower() for ch in seq)
    missing = sorted(set(by_contig)-seen)
    manifest = dict(release=110,assembly="GRCh38 primary assembly",policy="all gene biotypes except pseudogenes without transcribed_/translated_ prefix",biotypes=dict(biotypes),counts=dict(counts),missing_contigs=missing,reference_sha256=digest.hexdigest(),seconds=time.monotonic()-start,scn2a=[g for g in genes.values() if g["name"]=="SCN2A"],complete=not missing)
    (args.out / "manifest.json").write_text(json.dumps(manifest,indent=2)+"\n")
    print(json.dumps(manifest),flush=True)
    if missing:
        raise ValueError(f"Missing contigs: {missing}")


if __name__ == "__main__":
    main()
