use std::{fs, thread};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

fn main() {
    // Set the base path for your WasteFolder (replace with actual path)
    let waste_folder_path = PathBuf::from("/Users/ajstars/Developer/Projects/AutoWasteDeletor/rust/Temp");
    
    let deletion_threshold = Duration::new(2592000, 0); // 30 days in seconds

    loop {
        // Retrieve file paths in the WasteFolder
        let mut files = Vec::new();
        for entry in waste_folder_path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let file_path = entry.path();

            if file_path.is_file() {
                files.push(file_path.clone());
            }
        }

        // Filter files based on age
        let now = SystemTime::now();
        let files_to_delete = files.into_iter()
            .filter(|file_path| {
                let metadata = fs::metadata(file_path).unwrap();
                let creation_time = metadata.created().unwrap();
                creation_time + deletion_threshold <= now
            })
            .collect::<Vec<_>>();

        // Delete filtered files
        for file_path in files_to_delete {
            fs::remove_file(&file_path).unwrap_or_else(|err| {
                eprintln!("Error deleting {}: {}", file_path.display(), err);
            });
        }

        thread::sleep(Duration::from_secs(10000)); // Check 2.78 hours
    }
}





