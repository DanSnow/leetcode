use bstr::{BStr, ByteSlice};
use fjall::{KeyspaceCreateOptions, PersistMode, Slice};
use rootcause::prelude::*;
use std::fs;

type Result<T> = std::result::Result<T, Report>;

fn main() -> Result<()> {
    println!("Starting migration from sled to fjall...");

    // Open sled database
    let sled_db = sled::open(".stat/db").attach("fail to open sled db")?;

    // Create fjall database
    fs::create_dir_all(".stat/fjall").attach("fail to create fjall directory")?;
    let fjall_db = fjall::Database::builder(".stat/fjall")
        .open()
        .attach("fail to open fjall keyspace")?;
    let fjall_keyspace = fjall_db
        .keyspace("stats", KeyspaceCreateOptions::default)
        .attach("fail to open fjall partition")?;

    let mut count = 0;
    let mut errors = 0;

    // Iterate over sled entries and migrate to fjall
    for kv_result in sled_db.iter() {
        match kv_result {
            Ok((key, value)) => {
                let key_str = BStr::new(&key);
                let value_str = value.to_str().unwrap_or("<invalid utf8>");

                // Insert into fjall
                match fjall_keyspace.insert(Slice::new(key.as_bstr()), Slice::new(value.as_bstr()))
                {
                    Ok(_) => {
                        count += 1;
                        println!("✓ Migrated: {key_str} -> {value_str}");
                    }
                    Err(e) => {
                        errors += 1;
                        eprintln!("✗ Failed to insert {key_str}: {e}");
                    }
                }
            }
            Err(e) => {
                errors += 1;
                eprintln!("✗ Failed to read entry from sled: {e}");
            }
        }
    }

    // Persist fjall changes
    fjall_db
        .persist(PersistMode::SyncAll)
        .attach("fail to persist fjall data")?;

    println!("\nMigration complete!");
    println!("Successfully migrated: {count} entries");
    println!("Errors encountered: {errors}");

    if errors == 0 {
        println!("\nTo complete the migration:");
        println!("1. Verify the data in .stat/fjall");
        println!("2. Update your code to use fjall instead of sled");
        println!("3. Backup and remove the old .stat/db directory");
    }

    Ok(())
}
