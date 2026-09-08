use oofft::{
    Block, Record,
    summary::{CountUnit, Counts},
};
fn record(strand: &str, blocks: &[(usize, usize)]) -> Record {
    Record {
        id: "record".into(),
        sequence: String::new(),
        genes: vec!["intended".into(), "other".into()],
        transcripts: vec![],
        contig: "chr2".into(),
        strand: strand.into(),
        blocks: blocks
            .iter()
            .map(|&(start, end)| Block { start, end })
            .collect(),
    }
}
#[test]
fn genomic_counts_collapse_records_keep_junctions_and_take_minimum_edit_distance() {
    let intended = vec!["intended".into()];
    let mut counts = Counts::new(CountUnit::GenomicSite, true);
    let continuous = record("+", &[(100, 120)]);
    counts.add(&continuous, &intended, 0, 20, 3);
    counts.add(&continuous, &intended, 0, 20, 1);
    counts.add(&record("+", &[(100, 110), (110, 120)]), &intended, 0, 20, 2);
    assert_eq!(counts.by_edit_distance, [0, 1, 0, 0]);
    counts.add(&record("+", &[(100, 110), (115, 125)]), &intended, 0, 20, 2);
    counts.add(&record("-", &[(100, 120)]), &intended, 0, 20, 0);
    counts.add(&record("-", &[(110, 120), (100, 110)]), &intended, 0, 20, 0);
    assert_eq!(counts.by_edit_distance, [1, 1, 1, 0]);
    assert_eq!(counts.genes.into_iter().collect::<Vec<_>>(), vec!["other"]);
    let mut intervals = Counts::new(CountUnit::RecordInterval, false);
    for _ in 0..3 {
        intervals.add(&continuous, &intended, 0, 20, 1);
    }
    assert_eq!(intervals.by_edit_distance, [0, 3, 0, 0]);
    assert!(intervals.genes.is_empty());
}
