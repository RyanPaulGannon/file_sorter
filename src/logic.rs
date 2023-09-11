use std::fs;
use std::path::Path;

pub fn run_file_sorter(selected_directory: &Path) {
    if !selected_directory.is_dir() {
        eprintln!("Selected path is not a directory.");
        return;
    }

    if let Ok(entries) = fs::read_dir(selected_directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if let Some(file_name) = file_path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.contains("en") {
                            let new_folder_path = selected_directory.join("en_files");
                            if !new_folder_path.exists() {
                                if let Err(err) = fs::create_dir(&new_folder_path) {
                                    eprintln!("Unable to create new folder: {}", err);
                                    return;
                                }
                            }

                            let new_file_path = new_folder_path.join(file_name);
                            if let Err(err) = fs::rename(&file_path, &new_file_path) {
                                eprintln!("Error moving file: {}", err);
                            } else {
                                println!(
                                    "Moved {} to {}",
                                    file_name_str,
                                    new_folder_path.display()
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
