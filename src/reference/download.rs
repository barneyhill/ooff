use super::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
    time::{Duration, Instant},
};

#[derive(Serialize, Deserialize)]
pub struct Source {
    pub location: String,
    pub sha256: String,
    pub bytes: u64,
    pub ensembl_sum: Option<u16>,
}
pub fn fingerprint(path: &Path) -> Result<(String, u64, u16)> {
    let mut input = BufReader::new(File::open(path)?);
    digest(&mut input, &mut std::io::sink(), false)
}
fn digest(
    input: &mut impl Read,
    output: &mut impl Write,
    progress: bool,
) -> Result<(String, u64, u16)> {
    let mut hash = Sha256::new();
    let (mut bytes, mut checksum) = (0u64, 0u16);
    let mut buffer = [0u8; 65536];
    let mut last = Instant::now();
    loop {
        let n = input.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        for &b in &buffer[..n] {
            checksum = checksum.rotate_right(1).wrapping_add(u16::from(b));
        }
        hash.update(&buffer[..n]);
        output.write_all(&buffer[..n])?;
        bytes += n as u64;
        if progress && last.elapsed() > Duration::from_secs(10) {
            eprintln!("Downloaded {:.1} MiB", bytes as f64 / 1048576.0);
            last = Instant::now();
        }
    }
    Ok((format!("{:x}", hash.finalize()), bytes, checksum))
}
pub fn local(path: &Path) -> Result<Source> {
    let (sha256, bytes, _) = fingerprint(path)?;
    Ok(Source {
        location: fs::canonicalize(path)?.display().to_string(),
        sha256,
        bytes,
        ensembl_sum: None,
    })
}
fn expected(text: &str, filename: &str) -> Result<(u16, u64)> {
    let matches: Vec<_> = text
        .lines()
        .filter_map(|line| {
            let columns: Vec<_> = line.split_whitespace().collect();
            (columns.len() == 3 && columns[2] == filename).then_some(columns)
        })
        .collect();
    if matches.len() != 1 {
        return Err(format!("expected one checksum entry for {filename}").into());
    }
    Ok((matches[0][0].parse()?, matches[0][1].parse()?))
}
/// Downloads use HTTPS, provider BSD checksums, and recorded SHA-256 identities.
/// Incomplete downloads stay under unique .part names and are never activated.
pub fn fetch(url: &str, directory: &Path) -> Result<Source> {
    if !url.starts_with("https://ftp.ensembl.org/") {
        return Err("preset downloads require official Ensembl HTTPS URLs".into());
    }
    let (base, filename) = url.rsplit_once('/').ok_or("invalid download URL")?;
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(21600)))
        .timeout_connect(Some(Duration::from_secs(30)))
        .timeout_recv_response(Some(Duration::from_secs(60)))
        .build()
        .new_agent();
    let checksums = agent
        .get(format!("{base}/CHECKSUMS"))
        .call()?
        .body_mut()
        .read_to_string()?;
    let (sum, blocks) = expected(&checksums, filename)?;
    let path = directory.join(filename);
    let (sha256, bytes, actual) = if path.exists() {
        eprintln!("Checking cached {filename}");
        fingerprint(&path)?
    } else {
        eprintln!("Downloading {url}");
        let part = directory.join(format!("{filename}.part-{}", super::prepare::unique()));
        let mut response = agent
            .get(url)
            .header("Accept-Encoding", "identity")
            .call()?;
        let mut out = BufWriter::new(File::create_new(&part)?);
        let observed = digest(&mut response.body_mut().as_reader(), &mut out, true)?;
        out.flush()?;
        out.get_ref().sync_all()?;
        if observed.2 != sum || observed.1.div_ceil(1024) != blocks {
            return Err(format!(
                "download checksum mismatch: {url}; retained {}",
                part.display()
            )
            .into());
        }
        fs::rename(part, &path)?;
        observed
    };
    if actual != sum || bytes.div_ceil(1024) != blocks {
        return Err(format!(
            "cached download checksum mismatch: {}; move it aside before retrying",
            path.display()
        )
        .into());
    }
    Ok(Source {
        location: url.into(),
        sha256,
        bytes,
        ensembl_sum: Some(sum),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "downloads small checksum-verified README files from official Ensembl preset directories"]
    fn live_ensembl_https_and_checksums_for_all_presets() {
        let root = std::env::temp_dir().join(format!(
            "oofft-download-{}",
            super::super::prepare::unique()
        ));
        std::fs::create_dir(&root).unwrap();
        for preset in super::super::PRESETS {
            let directory = root.join(preset.name);
            std::fs::create_dir(&directory).unwrap();
            let urls = preset.urls();
            let base = urls[1].rsplit_once('/').unwrap().0;
            let source = fetch(&format!("{base}/README"), &directory).unwrap();
            assert!(source.bytes > 0);
            assert!(source.ensembl_sum.is_some());
            let cached = fetch(&format!("{base}/README"), &directory).unwrap();
            assert_eq!(source.sha256, cached.sha256);
            std::fs::write(directory.join("README"), b"corrupted cached download").unwrap();
            assert!(fetch(&format!("{base}/README"), &directory).is_err());
        }
    }

    #[test]
    fn bsd_sum_and_checksum_entry_are_exact() {
        let mut input = &b"hello\n"[..];
        let (_, bytes, sum) = digest(&mut input, &mut Vec::new(), false).unwrap();
        assert_eq!((bytes, sum), (6, 36979)); // sum -r, independently generated
        assert_eq!(
            expected("36979 1 fixture.fa.gz\n", "fixture.fa.gz").unwrap(),
            (36979, 1)
        );
        assert!(expected("1 1 other.fa.gz", "fixture.fa.gz").is_err());
        assert!(expected("1 1 x\n1 1 x", "x").is_err());
    }
}
