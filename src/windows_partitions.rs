use std::io::Error;

use crate::win_api::*;

#[derive(Debug)]
pub struct WindowsPartition {
    pub letter: char,
    pub ready: bool,
    pub name: String,
    pub size: u64,
    pub free_space: u64,
    pub file_system_name: String,
    pub drive_type: DriveType,
}

pub fn get_partitions() -> Result<Vec<WindowsPartition>, Error> {
    let drives = get_logical_drive()?;
    let mut result: Vec<WindowsPartition> = vec![];
    for letter in drives {
        let path = format!("{}:\\", letter);
        let drive_type = get_drive_type(path.to_string());
        let mut ready = true;
        let mut name = "".to_string();
        let mut total_size = 0;
        let mut free_space = 0;
        let mut file_system_name = "".to_string();
        match get_disk_free_space(path.to_string()) {
            Ok(value) => {
                total_size = value.1;
                free_space = value.2;
            }
            Err(err) => {
                if err.raw_os_error().is_some() &&
                    err.raw_os_error().unwrap() == 21 {
                    ready = false;
                } else {
                    return Err(err);
                }
            }
        };
        match get_volume_information(path.to_string()) {
            Ok(value) => {
                name = value.0;
                file_system_name = value.1;
            }
            Err(err) => {
                if err.raw_os_error().is_some() &&
                    err.raw_os_error().unwrap() == 21 {
                    ready = false;
                } else {
                    return Err(err);
                }
            }
        }
        result.push(WindowsPartition {
            letter,
            ready,
            name,
            size: total_size,
            free_space,
            file_system_name,
            drive_type,
        })
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_volume_name_test() {
        let res = get_partitions();
        for item in res.unwrap() {
            println!("{:?}", item)
        }
    }
}