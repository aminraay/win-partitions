use std::io::Error;

use crate::win_api::*;

#[derive(Debug)]
pub struct WindowsPartition {
    pub letter: char,
    pub name: String,
    pub size: u64,
    pub free_space: u64,
    pub file_system_name: String,
    pub drive_type: DriveType,
}

pub fn get_partitions() -> Result<Vec<WindowsPartition>, Error> {
    let drives = get_logical_drive()?;
    let mut result: Vec<WindowsPartition> = vec![];
    for volume in drives {
        let path = format!("{}:\\", volume);
        let volume_information = get_volume_information(path.to_string())?;
        let drive_type = get_drive_type(path.to_string());
        let disk_free_space = get_disk_free_space(path.to_string())?;
        result.push(WindowsPartition {
            name: volume_information.0,
            free_space: disk_free_space.2,
            letter: volume,
            size: disk_free_space.1,
            drive_type,
            file_system_name: volume_information.1,
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
