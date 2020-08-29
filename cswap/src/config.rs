use crate::ast::Script;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::ast::Sort;

use tempfile::Builder;
use tempfile::TempDir;

use crate::solver::ProfileIndex;
use log::warn;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug, Clone)]
pub struct Config {
    pub file_provider: FileProvider,
    pub remove_files: bool,
    pub max_iter: u32,
    pub rng_seed: u64,
    pub stack_size: usize,
    pub profiles: HashSet<ProfileIndex>,
    pub mask_size: usize,
    pub monitors_in_final: bool,
    pub enforce_on_resub: bool,
    pub use_bdom_vs: bool,
    pub max_const_relations_to_monitor: u8,
    pub skolemize_universal: bool,
    pub leaf_opt: bool,
    pub cp_og: bool,
}

#[macro_export]
macro_rules! liftio {
    ($x:expr) => {
        $x.map_err(|e| io::Error::new(io::ErrorKind::Other, format!("(Lifted) {:?}", e)))
    };
}

#[macro_export]
macro_rules! liftio_e {
    ($x:expr) => {
        io::Error::new(io::ErrorKind::Other, format!("(Lifted) {:?}", $x))
    };
}

impl Config {
    pub fn default() -> Self {
        Config {
            file_provider: FileProvider::new_tmp(),
            rng_seed: 0,
            max_iter: 20,
            stack_size: 50,
            remove_files: true,
            mask_size: 1,
            profiles: HashSet::new(),
            monitors_in_final: false,
            enforce_on_resub: false,
            use_bdom_vs: false,
            max_const_relations_to_monitor: 0,
            skolemize_universal: false,
            leaf_opt: false,
            cp_og: false,
        }
    }

    pub fn get_specific_seed<T: Hash>(&self, t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        self.rng_seed.hash(&mut s);
        s.finish()
    }

    pub fn to_csv_string(&self) -> String {
        format!(
            "{}, {}, {}, {}, {}, {}",
            self.max_iter,
            self.rng_seed,
            self.stack_size,
            self.mask_size,
            self.monitors_in_final,
            self.enforce_on_resub
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FileProvider {
    pub basedir: PathBuf,
    pub skeldir: PathBuf,
    pub cholesdir: PathBuf,
    pub mddir: PathBuf,
    pub scratchdir: PathBuf,
    pub bugdir: PathBuf,
}

impl FileProvider {
    pub fn new_tmp() -> FileProvider {
        FileProvider::new_existing(TempDir::new().unwrap().into_path().to_str().unwrap(), true)
    }
    pub fn new_existing(dirname: &str, existing: bool) -> FileProvider {
        let dirpath = Path::new(dirname).to_path_buf();
        let cholesdir = Self::get_subdir(&dirpath, "choles");
        let skeldir = Self::get_subdir(&dirpath, "skel");
        let mddir = Self::get_subdir(&dirpath, "md");
        let scratchdir = Self::get_subdir(&dirpath, "scratch");
        let bugdir = Self::get_subdir(&dirpath, "bugs");
        if !existing {
            fs::create_dir(&dirpath).unwrap();
        }
        fs::create_dir(&skeldir).unwrap();
        fs::create_dir(&cholesdir).unwrap();
        fs::create_dir(&mddir).unwrap();
        fs::create_dir(&scratchdir).unwrap();
        fs::create_dir(&bugdir).unwrap();
        FileProvider {
            basedir: dirpath,
            cholesdir: cholesdir,
            skeldir: skeldir,
            mddir: mddir,
            scratchdir: scratchdir,
            bugdir: bugdir,
        }
    }

    pub fn new(dirname: &str) -> FileProvider {
        FileProvider::new_existing(dirname, false)
    }

    pub fn og_w_monitors<'a>(&self, md: &'a mut Metadata) -> io::Result<PathBuf> {
        let tfile = Builder::new()
            .prefix("ogwms_")
            .rand_bytes(0)
            .suffix(&md.seed_file)
            .tempfile_in(&self.skeldir)?;
        let (_, pb) = liftio!(tfile.keep())?;
        md.og_w_monitors_skel_file = Some(name(&pb));
        Ok(pb)
    }

    pub fn cholesfile<'a>(&self, md: &'a mut Metadata) -> io::Result<PathBuf> {
        let tfile = Builder::new()
            .prefix("choles_")
            .rand_bytes(0)
            .suffix(&md.seed_file)
            .tempfile_in(&self.cholesdir)?;
        let (_, pb) = liftio!(tfile.keep())?;
        md.choles_file = name(&pb);
        Ok(pb)
    }

    pub fn skelfile<'a>(&self, md: &'a mut Metadata) -> io::Result<PathBuf> {
        let tfile = Builder::new()
            .prefix("skel_")
            .rand_bytes(2)
            .suffix(&format!("_{}", md.seed_file))
            .tempfile_in(&self.skeldir)?;
        let (_, pb) = liftio!(tfile.keep())?;
        md.skeleton_file = name(&pb);
        Ok(pb)
    }

    pub fn mdfile<'a>(&self, md: &'a mut Metadata) -> io::Result<PathBuf> {
        let tfile = Builder::new()
            .prefix("md_")
            .rand_bytes(2)
            .suffix(&format!("_{}", md.seed_file))
            .tempfile_in(&self.mddir)?;
        let (_, pb) = liftio!(tfile.keep())?;
        md.metadata_file = name(&pb);
        Ok(pb)
    }

    pub fn iterfile(&self, i: u64, md: &Metadata) -> io::Result<PathBuf> {
        let tfile = Builder::new()
            .prefix("iter_")
            .rand_bytes(10)
            .suffix(&format!("_{}_{}", i, md.seed_file))
            .tempfile_in(&self.scratchdir)?;
        let (_, pb) = liftio!(tfile.keep())?;
        Ok(pb)
    }

    pub fn resubfile(&self, iterfile: &Path, md: &Metadata) -> io::Result<PathBuf> {
        let stem = iterfile
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_owned())
            .unwrap_or(md.seed_file.to_owned() + "_unknown_iter");
        let tfile = Builder::new()
            .prefix("resub_")
            .rand_bytes(10)
            .suffix(&format!("_{}", stem))
            .tempfile_in(&self.scratchdir)?;
        let (_, pb) = liftio!(tfile.keep())?;
        Ok(pb)
    }

    pub fn bug_report(&self, buggy_file: &Path, report: &str) {
        let r = Builder::new()
            .prefix("bug_report_")
            .rand_bytes(0)
            .suffix(
                buggy_file
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("default_name"),
            )
            .tempfile_in(&self.bugdir)
            .and_then(|tf| {
                fs::write(tf.path(), report)?;
                liftio!(tf.keep())
            });
        match r {
            Err(e) => warn!("Error writing bug report for {:?}: {}", buggy_file, e),
            _ => (),
        }
    }

    pub fn cleanup_all(&self) {
        match fs::remove_dir_all(&self.basedir) {
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => (),
                _ => panic!(e),
            },
            _ => (),
        }
    }

    pub fn serialize_skel<'a, 'b>(
        &self,
        script: &'a Script,
        md: &'b mut Metadata,
    ) -> Result<PathBuf, io::Error> {
        let f = self.skelfile(md)?;
        fs::write(&f, script.to_string())?;
        Ok(f)
    }

    pub fn serialize_og_w_m<'a, 'b>(
        &self,
        script: &'a Script,
        md: &'b mut Metadata,
    ) -> Result<PathBuf, io::Error> {
        let f = self.og_w_monitors(md)?;
        fs::write(&f, script.to_string())?;
        Ok(f)
    }

    pub fn serialize_md<'a>(&self, md: &'a mut Metadata) -> Result<PathBuf, io::Error> {
        let f = self.mdfile(md)?;
        let mdstr = serde_lexpr::to_string(&md)?;
        fs::write(&f, mdstr)?;
        Ok(f)
    }

    pub fn serialize_skel_and_md<'a, 'b>(
        &self,
        script: &'a Script,
        md: &'b mut Metadata,
    ) -> Result<(PathBuf, PathBuf), io::Error> {
        let fs = self.serialize_skel(script, md)?;
        let fm = self.serialize_md(md)?;
        Ok((fs, fm))
    }

    pub fn serialize_iterfile_str(
        &self,
        script_str: &str,
        iteration: u64,
        md: &Metadata,
    ) -> Result<PathBuf, io::Error> {
        let f = self.iterfile(iteration, md)?;
        fs::write(&f, script_str)?;
        Ok(f)
    }

    pub fn serialize_iterfile(
        &self,
        script: &Script,
        iteration: u64,
        md: &Metadata,
    ) -> Result<PathBuf, io::Error> {
        let f = self.iterfile(iteration, md)?;
        fs::write(&f, script.to_string())?;
        Ok(f)
    }

    pub fn serialize_resub_str(
        &self,
        script_str: String,
        iterfile: &Path,
        md: &Metadata,
    ) -> Result<PathBuf, io::Error> {
        let f = self.resubfile(iterfile, md)?;
        fs::write(&f, script_str)?;
        Ok(f)
    }

    pub fn serialize_resub(
        &self,
        script: &Script,
        iterfile: &Path,
        md: &Metadata,
    ) -> Result<PathBuf, io::Error> {
        let f = self.resubfile(iterfile, md)?;
        fs::write(&f, script.to_string())?;
        Ok(f)
    }

    fn get_subdir(base: &Path, subname: &str) -> PathBuf {
        let mut subdir = base.to_path_buf();
        subdir.push(subname);
        subdir
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Metadata {
    pub bavns: Vec<(String, Sort)>,
    pub constvns: Vec<(String, Sort)>,
    pub seed_file: String,
    pub skeleton_file: String,
    pub og_w_monitors_skel_file: Option<String>,
    pub choles_file: String,
    pub metadata_file: String,
}
pub fn name(pb: &Path) -> String {
    pb.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("default_filename")
        .to_owned()
}

impl Metadata {
    pub fn new_empty() -> Self {
        Metadata {
            bavns: vec![],
            constvns: vec![],
            seed_file: "".to_owned(),
            skeleton_file: "".to_owned(),
            choles_file: "".to_owned(),
            metadata_file: "".to_owned(),
            og_w_monitors_skel_file: None,
        }
    }
    pub fn new(seed: &Path) -> Self {
        Metadata {
            bavns: vec![],
            constvns: vec![],
            seed_file: name(seed),
            choles_file: "".to_owned(),
            skeleton_file: "".to_owned(),
            metadata_file: "".to_owned(),
            og_w_monitors_skel_file: None,
        }
    }

    pub fn seed_path(&self, _fp: &FileProvider) -> PathBuf {
        panic!("Unimplemented!");
    }

    pub fn skel_path(&self, fp: &FileProvider) -> PathBuf {
        let mut p = fp.skeldir.clone();
        p.push(&self.skeleton_file);
        p
    }

    pub fn md_path(&self, fp: &FileProvider) -> PathBuf {
        let mut p = fp.mddir.clone();
        p.push(&self.metadata_file);
        p
    }

    pub fn choles_path(&self, fp: &FileProvider) -> PathBuf {
        let mut p = fp.cholesdir.clone();
        p.push(&self.choles_file);
        p
    }

    pub fn ogwms_path(&self, fp: &FileProvider) -> Option<PathBuf> {
        match &self.og_w_monitors_skel_file {
            Some(file) => {
                let mut p = fp.skeldir.clone();
                p.push(file);
                Some(p)
            }
            _ => None,
        }
    }
}
