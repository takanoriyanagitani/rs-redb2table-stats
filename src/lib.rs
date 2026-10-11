use std::io;
use std::path::Path;

use redb::ReadOnlyDatabase;
use redb::ReadableDatabase;
use redb::ReadableTableMetadata;
use redb::TableHandle;
use redb::TableStats;

pub trait HandleWithMeta: ReadableTableMetadata + TableHandle {}

impl<T> HandleWithMeta for T where T: ReadableTableMetadata + TableHandle {}

pub fn metadata_sink_simple(m: &dyn HandleWithMeta) -> Result<(), io::Error> {
    let stats: TableStats = m.stats().map_err(io::Error::other)?;
    let sz: u64 = m.len().map_err(io::Error::other)?;
    println!("stats: {stats:#?}");
    println!("len: {sz}");

    let name: &str = m.name();
    println!("name: {name}");

    Ok(())
}

pub struct Db(pub ReadOnlyDatabase);

impl Db {
    pub fn stats2sink<S>(&self, sink: &mut S) -> Result<(), io::Error>
    where
        S: FnMut(&dyn HandleWithMeta) -> Result<(), io::Error>,
    {
        let tx = self.0.begin_read().map_err(io::Error::other)?;
        let tables = tx.list_tables().map_err(io::Error::other)?;
        for hndl in tables {
            let untyped = tx.open_untyped_table(hndl).map_err(io::Error::other)?;
            sink(&untyped)?;
        }
        tx.close().map_err(io::Error::other)
    }

    pub fn stats2sink_default(&self) -> Result<(), io::Error> {
        self.stats2sink(&mut metadata_sink_simple)
    }
}

pub fn path2db<P>(p: P) -> Result<ReadOnlyDatabase, io::Error>
where
    P: AsRef<Path>,
{
    ReadOnlyDatabase::open(&p)
        .map_err(|e| format!("unable to open the db w/ path {:#?}: {e}", p.as_ref()))
        .map_err(io::Error::other)
}

pub fn path2db2stats2sink<P, S>(p: P, mut sink: S) -> Result<(), io::Error>
where
    P: AsRef<Path>,
    S: FnMut(&dyn HandleWithMeta) -> Result<(), io::Error>,
{
    let db: ReadOnlyDatabase = path2db(p)?;
    Db(db).stats2sink(&mut sink)
}

pub fn path2db2stats2sink_default<P>(p: P) -> Result<(), io::Error>
where
    P: AsRef<Path>,
{
    path2db2stats2sink(p, metadata_sink_simple)
}
