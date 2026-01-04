use fjall::{KeyspaceCreateOptions, PersistMode, Slice};
use jiff::Timestamp;
use rootcause::prelude::*;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
    os::unix::ffi::OsStrExt,
};

static TARGET_DIRS: [&str; 3] = ["easy", "medium", "hard"];

type Result<T> = std::result::Result<T, rootcause::Report>;

#[derive(Clone, Debug)]
struct StatReport {
    easy: BTreeMap<String, i32>,
    medium: BTreeMap<String, i32>,
    hard: BTreeMap<String, i32>,
}

fn main() -> Result<()> {
    fs::create_dir_all(".stat/fjall").attach("fail to create dir")?;
    let fjall_db = fjall::Database::builder(".stat/fjall")
        .open()
        .attach("fail to open fjall database")?;
    let db = fjall_db
        .keyspace("stats", KeyspaceCreateOptions::default)
        .attach("fail to open fjall keyspace")?;

    let mut report = StatReport {
        easy: BTreeMap::new(),
        hard: BTreeMap::new(),
        medium: BTreeMap::new(),
    };

    for dir in TARGET_DIRS {
        let entries = fs::read_dir(format!("src/{dir}")).attach("fail to read directory")?;
        for entry in entries {
            let entry = entry.attach("fail to read directory entry")?;
            let file_name = entry.file_name();
            let filename = file_name.as_bytes();
            if filename == b"mod.rs" {
                continue;
            }
            let value = db.get(filename).attach("fail to get key")?;
            let date = match value {
                Some(value) => String::from_utf8(value.to_vec()).attach("fail to convert to string")?,
                None => {
                    let meta = entry.metadata().attach("fail to read meta")?;
                    let created_time = meta.created().attach("fail to get created time")?;
                    let timestamp = Timestamp::try_from(created_time).attach("fail to convert to timestamp")?;
                    let datetime = timestamp.to_zoned(jiff::tz::TimeZone::system());

                    let created_str = datetime.strftime("%Y-%m-%d").to_string();

                    db.insert(Slice::new(filename), Slice::new(created_str.as_bytes()))
                        .attach("fail to insert record")?;
                    created_str
                }
            };
            let storage = match dir {
                "easy" => &mut report.easy,
                "medium" => &mut report.medium,
                "hard" => &mut report.hard,
                _ => unreachable!(),
            };
            storage
                .entry(date)
                .and_modify(|i| {
                    *i += 1;
                })
                .or_insert(1);
        }
    }

    // Persist changes to disk
    fjall_db
        .persist(PersistMode::SyncAll)
        .attach("fail to persist fjall data")?;

    let mut f = File::create("STAT.md").attach("fail to create stat.md")?;
    for dir in TARGET_DIRS {
        writeln!(f, "{}\n===\n", dir).attach("fail to write to stat.md")?;
        let storage = match dir {
            "easy" => &report.easy,
            "medium" => &report.medium,
            "hard" => &report.hard,
            _ => unreachable!(),
        };
        if storage.is_empty() {
            writeln!(f, "N/A").attach("fail to write to stat.md")?;
        } else {
            writeln!(f, "| date | finished |\n| --- | --- |").attach("fail to write to stat.md")?;
            to_report_table(&mut f, storage)?;
        }
        writeln!(f).attach("fail to write to stat.md")?;
    }

    Ok(())
}

fn to_report_table(f: &mut File, map: &BTreeMap<String, i32>) -> Result<()> {
    for (k, v) in map.iter() {
        writeln!(f, "| {} | {} |", k, v).attach("fail to write to stat.md")?;
    }
    Ok(())
}
