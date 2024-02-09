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










// May work on this in the future to make it more efficient

// use std::fs;
// use std::path::Path;
// use std::time::{Duration, SystemTime};
// use log::{debug, error, info};

// fn main() {
//     // Set up logging (replace with your preferred logging configuration)
//     env_logger::init();

//     // Set the base path for your WasteFolder (replace with actual path)
//     let waste_folder_path = Path::new("/Users/ajstars/Developer/Projects/AutoWasteDeletor/rust/Temp");

//     // Set the deletion threshold (e.g., 30 days)
//     // let deletion_threshold = Duration::new(2592000, 0); // 30 days in seconds
//     let deletion_threshold = Duration::new(10, 0); // 30 days in seconds

//     // Get the current time
//     let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

//     // Process files in the WasteFolder
//     loop {
//         process_files(waste_folder_path, deletion_threshold, current_time)
//             .unwrap_or_else(|err| error!("Error processing WasteFolder: {}", err));

//         // Pause to avoid excessive system checks (adjust as needed)
//         std::thread::sleep(Duration::from_secs(1)); // Check every minute
//     }
// }

// fn process_files(
//     waste_folder_path: &Path,
//     deletion_threshold: Duration,
//     current_time: Duration,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // Iterate through files in the WasteFolder
//     for entry in waste_folder_path.read_dir()? {
//         print!("entry: {:?}", entry);
//         let entry = entry?;
//         let file_path = entry.path();

//         // Check if it's a file
//         if !file_path.is_file() {
//             debug!("Skipping non-file entry: {}", file_path.display());
//             continue;
//         }

//         // Get file modification time
//         match file_path.metadata() {
//             Ok(metadata) => {
//                 let file_mtime = metadata.modified()?.duration_since(SystemTime::UNIX_EPOCH)?;

//                 // Check if file is older than the deletion threshold
//                 if current_time - file_mtime > deletion_threshold {
//                     // Delete the file
//                     delete_file(&file_path)?;
//                 }
//             }
//             Err(error) => {
//                 error!("Error getting metadata for {}: {}", file_path.display(), error);
//             }
//         }
//     }

//     Ok(())
// }

// fn delete_file(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
//     fs::remove_file(file_path)?;
//     info!("Deleted: {}", file_path.display());
//     Ok(())
// }
