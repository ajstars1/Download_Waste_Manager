use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};
use log::{debug, error, info};

fn main() {
    // Set up logging (replace with your preferred logging configuration)
    env_logger::init();

    // Set the base path for your WasteFolder (replace with actual path)
    let waste_folder_path = Path::new("/Users/ajstars/Developer/Projects/AutoWasteDeletor/Rust/Temp");

    // Set the deletion threshold (e.g., 30 days)
    // let deletion_threshold = Duration::new(2592000, 0); // 30 days in seconds
    let deletion_threshold = Duration::new(10, 0); // 30 days in seconds

    // Get the current time
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    // Process files in the WasteFolder
    process_files(waste_folder_path, deletion_threshold, current_time)
        .unwrap_or_else(|err| error!("Error processing WasteFolder: {}", err));
}

fn process_files(
    waste_folder_path: &Path,
    deletion_threshold: Duration,
    current_time: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    // Iterate through files in the WasteFolder
    for entry in waste_folder_path.read_dir()? {
        let entry = entry?;
        let file_path = entry.path();

        // Check if it's a file
        if !file_path.is_file() {
            debug!("Skipping non-file entry: {}", file_path.display());
            continue;
        }

        // Get file modification time
        match file_path.metadata() {
            Ok(metadata) => {
                let file_mtime = metadata.modified()?.duration_since(SystemTime::UNIX_EPOCH)?;

                // Check if file is older than the deletion threshold
                if current_time - file_mtime > deletion_threshold {
                    // Delete the file
                    delete_file(&file_path)?;
                }
            }
            Err(error) => {
                error!("Error getting metadata for {}: {}", file_path.display(), error);
            }
        }
    }

    Ok(())
}

fn delete_file(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_file(file_path)?;
    info!("Deleted: {}", file_path.display());
    Ok(())
}
















// Extra code for handling folders

// fn process_files(
//     waste_folder_path: &Path,
//     deletion_threshold: Duration,
//     current_time: SystemTime,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // ... existing code

//     for entry in waste_folder_path.read_dir()? {
//         let entry = entry?;
//         let file_path = entry.path();

//         if file_path.is_dir() {
//             // Handle folders separately
//             delete_folder_if_empty(&file_path, deletion_threshold, current_time)?;
//         } else {
//             // Existing file processing logic
//             delete_file(&file_path)?;
//         }
//     }

//     Ok(())
// }

// fn delete_folder_if_empty(
//     folder_path: &Path,
//     deletion_threshold: Duration,
//     current_time: SystemTime,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // Check if folder is empty
//     if folder_path.read_dir()?.next().is_none() {
//         // Check if folder is older than threshold
//         if folder_path.metadata()?.modified()? + deletion_threshold < current_time {
//             // Delete empty folder
//             fs::remove_dir_all(folder_path)?;
//             info!("Deleted empty folder: {}", folder_path.display());
//         } else {
//             debug!("Folder not empty or not old enough: {}", folder_path.display());
//         }
//     } else {
//         debug!("Skipping non-empty folder: {}", folder_path.display());
//     }

//     Ok(())
// }
