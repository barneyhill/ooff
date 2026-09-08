//! Count-only aggregation. No alignments, site JSON, or thermodynamic scoring.
use crate::Record;
use rustc_hash::FxHashMap;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum CountUnit {
    /// Collapse identical contig, strand and ordered genomic blocks.
    GenomicSite,
    /// Count each distinct record/start/end interval independently.
    RecordInterval,
}
impl CountUnit {
    pub fn name(self) -> &'static str {
        match self {
            Self::GenomicSite => "genomic-site",
            Self::RecordInterval => "record-interval",
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct GenomicSite {
    contig: usize,
    reverse: bool,
    first: (usize, usize),
    rest: Vec<(usize, usize)>,
}

pub struct Counts {
    pub by_edit_distance: [u64; 4],
    pub genes: BTreeSet<String>,
    unit: CountUnit,
    include_genes: bool,
    contigs: FxHashMap<String, usize>,
    sites: FxHashMap<GenomicSite, usize>,
}
impl Counts {
    pub fn new(unit: CountUnit, include_genes: bool) -> Self {
        Self {
            by_edit_distance: [0; 4],
            genes: BTreeSet::new(),
            unit,
            include_genes,
            contigs: FxHashMap::default(),
            sites: FxHashMap::default(),
        }
    }
    pub fn total(&self) -> u64 {
        self.by_edit_distance.iter().sum()
    }
    pub fn needs_annotation(&self) -> bool {
        self.include_genes || self.unit == CountUnit::GenomicSite
    }
    pub fn add_interval(&mut self, distance: usize) {
        self.by_edit_distance[distance] += 1;
    }
    pub fn add(
        &mut self,
        record: &Record,
        intended: &[String],
        start: usize,
        end: usize,
        distance: usize,
    ) {
        if self.include_genes {
            for gene in &record.genes {
                if !intended.contains(gene) && !self.genes.contains(gene) {
                    self.genes.insert(gene.clone());
                }
            }
        }
        if self.unit == CountUnit::RecordInterval {
            self.add_interval(distance);
            return;
        }
        let next = self.contigs.len();
        let contig = match self.contigs.get(&record.contig) {
            Some(&id) => id,
            None => {
                self.contigs.insert(record.contig.clone(), next);
                next
            }
        };
        let mut blocks = record.mapped_blocks(start, end).map(|b| (b.start, b.end));
        let mut key = GenomicSite {
            contig,
            reverse: record.strand == "-",
            first: blocks.next().expect("validated nonempty site"),
            rest: Vec::new(),
        };
        // Adjacent annotation blocks represent the same continuous genomic span.
        for block in blocks {
            let previous = key.rest.last_mut().unwrap_or(&mut key.first);
            if (!key.reverse && previous.1 == block.0) || (key.reverse && previous.0 == block.1) {
                previous.0 = previous.0.min(block.0);
                previous.1 = previous.1.max(block.1);
            } else {
                key.rest.push(block);
            }
        }
        match self.sites.entry(key) {
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert(distance);
                self.by_edit_distance[distance] += 1;
            }
            std::collections::hash_map::Entry::Occupied(mut slot) => {
                let old = *slot.get();
                if distance < old {
                    self.by_edit_distance[old] -= 1;
                    self.by_edit_distance[distance] += 1;
                    slot.insert(distance);
                }
            }
        }
    }
}
