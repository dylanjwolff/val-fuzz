use crate::ast::Script;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use tempfile::Builder;

#[macro_use]
use serde::{Serialize, Deserialize};
use crate::solver::ProfileIndex;
use log::warn;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug, Clone)]
pub struct Config {
    pub file_provider: FileProvider,
    pub max_iter: u32,
    pub rng_seed: u64,
    pub stack_size: usize,
    pub profiles: HashSet<ProfileIndex>,
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
    pub fn get_specific_seed<T: Hash>(&self, t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        self.rng_seed.hash(&mut s);
        s.finish()
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
    pub fn new(dirname: &str) -> FileProvider {
        let dirpath = Path::new(dirname).to_path_buf();
        let cholesdir = Self::get_subdir(&dirpath, "choles");
        let skeldir = Self::get_subdir(&dirpath, "skel");
        let mddir = Self::get_subdir(&dirpath, "md");
        let scratchdir = Self::get_subdir(&dirpath, "scratch");
        let bugdir = Self::get_subdir(&dirpath, "bugs");
        fs::create_dir(&dirpath).unwrap();
        fs::create_dir(&skeldir).unwrap();
        fs::create_dir(&cholesdir).unwrap();
        fs::create_dir(&mddir).unwrap();
        fs::create_dir(&scratchdir).unwrap();
        fs::create_dir(&bugdir).unwrap();
        FileProvider {
            basedir: dirpath,
            cholesdir:cholesdir,
            skeldir: skeldir,
            mddir: mddir,
            scratchdir: scratchdir,
            bugdir: bugdir,
        }
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
            .rand_bytes(0)
            .suffix(&md.seed_file)
            .tempfile_in(&self.skeldir)?;
        let (_, pb) = liftio!(tfile.keep())?;
        md.skeleton_file = name(&pb);
        Ok(pb)
    }

    pub fn mdfile<'a>(&self, md: &'a mut Metadata) -> io::Result<PathBuf> {
        let tfile = Builder::new()
            .prefix("md_")
            .rand_bytes(0)
            .suffix(&md.seed_file)
            .tempfile_in(&self.mddir)?;
        let (_, pb) = liftio!(tfile.keep())?;
        md.metadata_file = name(&pb);
        Ok(pb)
    }

    pub fn iterfile(&self, md: &Metadata) -> io::Result<PathBuf> {
        let tfile = Builder::new()
            .prefix("iter_")
            .rand_bytes(10)
            .suffix(&md.seed_file)
            .tempfile_in(&self.scratchdir)?;
        let (_, pb) = liftio!(tfile.keep())?;
        Ok(pb)
    }

    pub fn resubfile(&self, md: &Metadata) -> io::Result<PathBuf> {
        let tfile = Builder::new()
            .prefix("resub_")
            .rand_bytes(10)
            .suffix(&md.seed_file)
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

    pub fn serialize_resub(&self, script: &Script, md: &Metadata) -> Result<PathBuf, io::Error> {
        let f = self.resubfile(md)?;
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
    pub bavns: Vec<String>,
    pub constvns: Vec<String>,
    pub seed_file: String,
    pub skeleton_file: String,
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
}
