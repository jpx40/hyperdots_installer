use alpm::{self, Alpm, Package};
use std::fmt::Result;

pub struct PkgDB {
    db: Alpm,
}

impl PkgDB {
    pub fn init() -> alpm::Result<PkgDB> {
        Ok(PkgDB {
            db: Alpm::new("/", "/var/lib/pacman")?,
        })
    }
    pub fn is_installed(&mut self, pkg: String) -> bool {
        let db = self.db.localdb();
        let pkg = db.pkg(pkg);
        pkg.is_ok()
    }
}
struct Pkg {}
pub fn init() -> alpm::Result<Alpm> {
    Alpm::new("/", "/var/lib/pacman")
}

fn check_if_installed(alpm: &Alpm, pkg: String) -> bool {
    let db = alpm.localdb();
    let pkg = db.pkg(pkg);
    pkg.is_ok()
}
