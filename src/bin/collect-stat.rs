use fjall::{KeyspaceCreateOptions, PersistMode, Slice};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
    os::unix::ffi::OsStrExt,
};

static TARGET_DIRS: [&str; 3] = ["easy", "medium", "hard"];

#[derive(Clone, Debug)]
struct Report {
    easy: BTreeMap<String, i32>,
    medium: BTreeMap<String, i32>,
    hard: BTreeMap<String, i32>,
}

fn main() {
    fs::create_dir_all(".stat/fjall").expect("fail to create dir");
    let fjall_db = fjall::Database::builder(".stat/fjall")
        .open()
        .expect("fail to open fjall database");
    let db = fjall_db
        .keyspace("stats", KeyspaceCreateOptions::default)
        .expect("fail to open fjall keyspace");

    let mut report = Report {
        easy: BTreeMap::new(),
        hard: BTreeMap::new(),
        medium: BTreeMap::new(),
    };

    for dir in TARGET_DIRS {
        if let Ok(entries) = fs::read_dir(format!("src/{dir}")) {
            for entry in entries {
                let entry = entry.unwrap();
                let file_name = entry.file_name();
                let filename = file_name.as_bytes();
                if filename == b"mod.rs" {
                    continue;
                }
                let value = db.get(filename).expect("fail to get key");
                let date = match value {
                    Some(value) => {
                        String::from_utf8(value.to_vec()).expect("fail to convert to string")
                    }
                    None => {
                        let meta = entry.metadata().expect("fail to read meta");
                        let created = time::OffsetDateTime::from(
                            meta.created().expect("fail to get created time"),
                        );

                        let created_str = format!(
                            "{}-{:02}-{:02}",
                            created.year(),
                            created.month() as u8,
                            created.day()
                        );

                        db.insert(Slice::new(filename), Slice::new(created_str.as_bytes()))
                            .expect("fail to insert record");
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
    }

    // Persist changes to disk
    fjall_db
        .persist(PersistMode::SyncAll)
        .expect("fail to persist fjall data");

    let mut f = File::create("STAT.md").expect("Fail to create stat.md");
    for dir in TARGET_DIRS {
        writeln!(f, "{}\n===\n", dir).unwrap();
        let storage = match dir {
            "easy" => &report.easy,
            "medium" => &report.medium,
            "hard" => &report.hard,
            _ => unreachable!(),
        };
        if storage.is_empty() {
            writeln!(f, "N/A").unwrap();
        } else {
            writeln!(f, "| date | finished |\n| --- | --- |").unwrap();
            to_report_table(&mut f, storage);
        }
        writeln!(f).unwrap();
    }
}

fn to_report_table(f: &mut File, map: &BTreeMap<String, i32>) {
    for (k, v) in map.iter() {
        writeln!(f, "| {} | {} |", k, v).expect("Fail to write");
    }
}
