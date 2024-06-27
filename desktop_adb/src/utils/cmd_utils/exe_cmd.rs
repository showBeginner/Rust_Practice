use std::process::{Command, Output};
use std::str::Split;
use std::thread;

pub(crate) fn install_apk_for_all_devices(devices: Vec<String>, files: Vec<String>){
    let mut threads = Vec::new();
    for device in devices.iter() {
        for file in files.iter() {
            let device_cp = device.clone();
            let file_cp = file.clone();
            let thread = thread::spawn(move || {
                let command = format!("adb -s {device_cp} install -t {file_cp}");
                let result = execute_adb(&command);
                match result.status.success() {
                    true => println!("APK installed successfully on device {}", device_cp),
                    false => panic!("Failed to install APK on device {}: {:?}", device_cp, result.stderr),
                }
            });
            threads.push(thread);
        }
    }
    for thread in threads {
        thread.join().unwrap();
    }
}

pub(crate) fn root_devices(devices: Vec<String>) {
    let mut threads = Vec::new();
    for device in devices.iter() {
        let device_cp = device.clone();
        let thread = thread::spawn(move || {
            let command = format!("adb -s {device_cp} root ");
            let result = execute_adb(&command);
            match result.status.success() {
                true => println!("Root on device {}", device_cp),
                false => panic!("Failed Root on device {}: {:?}", device_cp, result.stderr),
            }
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}

pub(crate) fn get_devices_via_adb() -> Vec<String> {
    let result = execute_adb("adb devices");
    let binding = String::from_utf8_lossy(&result.stdout).to_string();
    let lines = binding.split("\n");
    let serial = devices_sn(lines);
    serial.iter().map(|s| String::from(*s)).collect()
}

fn devices_sn<'a>(lines: Split<'a, &str>) -> Vec<&'a str> {
    let mut devices = vec![];
    for line in lines {
        if line.contains("\tdevice") {
            let sns = line.split("\t");
            for sn in sns {
                if !sn.contains("device") {
                    devices.push(sn);
                }
            }
        }
    }
    return devices
}

fn execute_adb(com_str: &str) -> Output {
    let result = if cfg!(target_os = "windows"){
        Command::new("cmd")
            .args(["/C", com_str])
            .output()
            .expect("Failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(com_str)
            .output()
            .expect("Failed to execute process")
    };
    return result;
}
