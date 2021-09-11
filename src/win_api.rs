use std::io::{Error};

use crate::bindings::{
    Windows::Win32::Foundation::PWSTR,
    Windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW,
    Windows::Win32::Storage::FileSystem::GetDriveTypeW,
    Windows::Win32::Storage::FileSystem::GetLogicalDrives,
    Windows::Win32::Storage::FileSystem::GetVolumeInformationW,
};

fn vec_u16_to_string(vec: &Vec<u16>) -> String {
    let mut index = 0;
    for item in 0..vec.len() {
        if vec[item] == 0 {
            break;
        }
        index = item + 1;
    }
    String::from_utf16_lossy(&vec[0..index])
}

#[derive(Debug)]
pub enum DriveType {
    DriveUnknown = 0,
    DriveNoRootDir = 1,
    DriveRemovable = 2,
    DriveFixed = 3,
    DriveRemote = 4,
    DriveCDRom = 5,
    DriveRamDisk = 6,
}

impl From<u32> for DriveType {
    fn from(index: u32) -> Self {
        match index {
            0 => DriveType::DriveUnknown,
            1 => DriveType::DriveNoRootDir,
            2 => DriveType::DriveRemovable,
            3 => DriveType::DriveFixed,
            4 => DriveType::DriveRemote,
            5 => DriveType::DriveCDRom,
            6 => DriveType::DriveRamDisk,
            _ => {
                panic!("Invalid Drive Type")
            }
        }
    }
}

pub fn get_volume_information(
    lprootpathname: String
) -> Result<(String, String, u32, u32, u32), Error> {
    // Maximum Volume name length is 32 characters which is equivalent to 64 unicode bytes
    let mut volume_name_buf: Vec<u16> = Vec::with_capacity(64);
    volume_name_buf.resize(64, 0);

    let mut file_system_name_buf: Vec<u16> = Vec::with_capacity(255);
    file_system_name_buf.resize(255, 0);

    let pwstr_volume_name: PWSTR = PWSTR(volume_name_buf.as_mut_ptr());
    let pwstr_file_system_name: PWSTR = PWSTR(file_system_name_buf.as_mut_ptr());

    let mut lpvolumeserialnumber: u32 = 0;
    let mut lpmaximumcomponentlength: u32 = 0;
    let mut lpfilesystemflags: u32 = 0;
    let result = unsafe {
        GetVolumeInformationW(
            lprootpathname,
            pwstr_volume_name,
            volume_name_buf.capacity() as u32,
            &mut lpvolumeserialnumber,
            &mut lpmaximumcomponentlength,
            &mut lpfilesystemflags,
            pwstr_file_system_name,
            file_system_name_buf.capacity() as u32).as_bool()
    };

    if result {
        let result_volume_name = vec_u16_to_string(&volume_name_buf);
        let result_volume_system_name = vec_u16_to_string(&file_system_name_buf);
        Ok((result_volume_name, result_volume_system_name, lpvolumeserialnumber, lpmaximumcomponentlength, lpfilesystemflags))
    } else {
        Err(Error::last_os_error())
    }
}

pub fn get_drive_type(
    lprootpathname: String,
) -> DriveType {
    let result = unsafe {
        GetDriveTypeW(
            lprootpathname
        )
    };

    DriveType::from(result)
}

pub fn get_disk_free_space(
    lpdirectoryname: String
) -> Result<(u64, u64, u64), Error> {
    let mut lpfreebytesavailabletocaller: u64 = 0;
    let mut lptotalnumberofbytes: u64 = 0;
    let mut lptotalnumberoffreebytes: u64 = 0;
    let result =
        unsafe {
            GetDiskFreeSpaceExW(
                lpdirectoryname,
                &mut lpfreebytesavailabletocaller,
                &mut lptotalnumberofbytes,
                &mut lptotalnumberoffreebytes).as_bool()
        };

    if result {
        Ok((lpfreebytesavailabletocaller, lptotalnumberofbytes, lptotalnumberoffreebytes))
    } else {
        Err(Error::last_os_error())
    }
}

/// Use [GetLogicalDrives](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getlogicaldrives) Windows API function
/// and returns Vector of drive letters
pub fn get_logical_drive() -> Vec<char> {
    let logical_drive_decimal = unsafe { GetLogicalDrives() };
    let mut mask = 1;
    let mut result: Vec<char> = vec![];

    for index in 1..26 {
        if mask & logical_drive_decimal == mask {
            let char = std::char::from_u32(index + 64);
            result.push(char.unwrap());
        }
        mask = mask << 1;
    }

    result
}