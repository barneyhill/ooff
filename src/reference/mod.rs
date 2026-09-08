//! Versioned reference presets and immutable, locally prepared reference bundles.
mod build;
mod download;
mod prepare;
pub use build::{Gene, build_reference};
pub use prepare::{Cli, run};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs::File,
    path::{Path, PathBuf},
};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub const RELEASE: u32 = 110;
pub const BIOTYPES: &str =
    "all gene biotypes except pseudogenes without transcribed_/translated_ prefix";

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Preset {
    pub name: &'static str,
    pub species: &'static str,
    pub label: &'static str,
    pub assembly: &'static str,
    pub assembly_scope: &'static str,
}
pub const PRESETS: &[Preset] = &[
    Preset {
        name: "hg38",
        species: "homo_sapiens",
        label: "Human",
        assembly: "GRCh38",
        assembly_scope: "primary_assembly",
    },
    Preset {
        name: "mm39",
        species: "mus_musculus",
        label: "Mouse",
        assembly: "GRCm39",
        assembly_scope: "primary_assembly",
    },
    Preset {
        name: "rn7",
        species: "rattus_norvegicus",
        label: "Rat",
        assembly: "mRatBN7.2",
        assembly_scope: "toplevel",
    },
    Preset {
        name: "cyno",
        species: "macaca_fascicularis",
        label: "Cynomolgus macaque",
        assembly: "Macaca_fascicularis_6.0",
        assembly_scope: "toplevel",
    },
    Preset {
        name: "rhesus",
        species: "macaca_mulatta",
        label: "Rhesus macaque",
        assembly: "Mmul_10",
        assembly_scope: "toplevel",
    },
];
pub fn preset(name: &str) -> Option<Preset> {
    let name = name.to_ascii_lowercase();
    let canonical = match name.as_str() {
        "human" | "grch38" => "hg38",
        "mouse" | "grcm39" => "mm39",
        "rat" | "mratbn7.2" => "rn7",
        "cynomolgus" | "macaca_fascicularis_6.0" => "cyno",
        "mmul_10" => "rhesus",
        other => other,
    };
    PRESETS.iter().find(|p| p.name == canonical).copied()
}
impl Preset {
    pub fn key(self) -> String {
        format!("{}-ensembl{RELEASE}-v1", self.name)
    }
    pub fn urls(self) -> [String; 2] {
        let mut organism = self.species.to_owned();
        organism[..1].make_ascii_uppercase();
        [
            format!(
                "https://ftp.ensembl.org/pub/release-{RELEASE}/fasta/{}/dna/{organism}.{}.dna_sm.{}.fa.gz",
                self.species, self.assembly, self.assembly_scope
            ),
            format!(
                "https://ftp.ensembl.org/pub/release-{RELEASE}/gtf/{}/{organism}.{}.{RELEASE}.gtf.gz",
                self.species, self.assembly
            ),
        ]
    }
}
pub fn cache_dir(explicit: Option<&Path>) -> Result<PathBuf> {
    if let Some(path) = explicit {
        return Ok(path.into());
    }
    if let Some(path) = std::env::var_os("OOFFT_CACHE_DIR") {
        return Ok(path.into());
    }
    if let Some(path) = std::env::var_os("XDG_CACHE_HOME") {
        return Ok(PathBuf::from(path).join("oofft/references"));
    }
    let home = std::env::var_os("HOME")
        .ok_or("set --cache-dir or OOFFT_CACHE_DIR to a writable reference cache")?;
    Ok(PathBuf::from(home).join(".cache/oofft/references"))
}
pub fn key(name: &str) -> Result<String> {
    if let Some(p) = preset(name) {
        return Ok(p.key());
    }
    if name.is_empty()
        || name.len() > 100
        || !name
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
    {
        return Err("reference name must contain only letters, numbers, '-' or '_'".into());
    }
    Ok(format!("custom-{name}-v1"))
}

#[derive(Serialize, Deserialize)]
pub struct Bundle {
    pub format: String,
    pub name: String,
    pub reference_release: String,
    pub scope: String,
    pub biotype_policy: String,
    pub source_files: Vec<download::Source>,
    pub reference_sha256: String,
    pub genes_sha256: String,
    pub annotations_sha256: String,
    pub records: usize,
    pub bases: usize,
    pub unknown_bases: usize,
}
#[derive(Serialize, Deserialize)]
struct Current {
    directory: String,
}

pub fn resolve(name: &str, cache: &Path) -> Result<(PathBuf, Bundle)> {
    let root = cache.join(key(name)?);
    let marker = root.join("current.json");
    if !marker.exists() {
        return Err(format!("reference '{name}' is not prepared; run: oofft reference prepare {name} --cache-dir {}", cache.display()).into());
    }
    let current: Current = serde_json::from_reader(File::open(marker)?)?;
    if !current.directory.starts_with("build-")
        || !current
            .directory
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
    {
        return Err("invalid reference bundle directory".into());
    }
    let directory = root.join(current.directory);
    let bundle: Bundle = serde_json::from_reader(File::open(directory.join("bundle.json"))?)?;
    if bundle.format != "oofft-reference-v1" || bundle.name != preset(name).map_or(name, |p| p.name)
    {
        return Err("reference bundle identity mismatch".into());
    }
    for filename in [
        "reference.fa",
        "records.jsonl",
        "genes.json",
        "index/manifest.json",
        "annotation-cache.json",
    ] {
        if !directory.join(filename).is_file() {
            return Err(format!("incomplete reference bundle: missing {filename}").into());
        }
    }
    if let Some(p) = preset(name)
        && bundle.reference_release != format!("Ensembl{RELEASE}-{}", p.assembly)
    {
        return Err("reference bundle release mismatch".into());
    }
    Ok((directory, bundle))
}

pub fn resolve_exclusions(
    names: &[String],
    genes: &BTreeMap<String, String>,
) -> Result<Vec<String>> {
    let mut resolved = BTreeSet::new();
    for name in names {
        if name.trim().is_empty() {
            return Err("--exclude requires a gene symbol or ID".into());
        }
        let exact: Vec<_> = genes
            .keys()
            .filter(|id| {
                id.eq_ignore_ascii_case(name)
                    || unversioned(id).eq_ignore_ascii_case(unversioned(name))
            })
            .collect();
        let matches = if exact.is_empty() {
            genes
                .iter()
                .filter(|(_, symbol)| !symbol.is_empty() && symbol.eq_ignore_ascii_case(name))
                .map(|(id, _)| id)
                .collect()
        } else {
            exact
        };
        match matches.as_slice() {
            [] => return Err(format!("unknown excluded gene '{name}' in this reference; use a gene ID present in its annotation").into()),
            [id] => { resolved.insert((*id).clone()); },
            _ => return Err(format!("ambiguous gene symbol '{name}'; use one of: {}", matches.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")).into()),
        }
    }
    Ok(resolved.into_iter().collect())
}

fn unversioned(id: &str) -> &str {
    match id.rsplit_once('.') {
        Some((base, version))
            if !version.is_empty() && version.bytes().all(|b| b.is_ascii_digit()) =>
        {
            base
        }
        _ => id,
    }
}
