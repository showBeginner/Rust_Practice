use crate::utils::cmd_utils::exe_cmd;
use native_dialog::FileDialog;

pub(crate) enum Adbcommand {
    Root,
    Devices,
    Select,
    Install,
}


pub(crate) struct AdbCli{
    devices: Vec<String>,
    files: Vec<String>,
}

impl AdbCli{

    pub(crate) fn new() -> AdbCli{
        let client = AdbCli {
            devices: exe_cmd::get_devices_via_adb(),
            files: vec![],
        };
        exe_cmd::root_devices(client.get_devices());
        client
    }

    pub(crate) fn get_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub(crate) fn get_files(&self) -> Vec<String> {
        self.files.clone()
    }

    pub(crate) fn clear_files(&mut self) {
        self.files.clear();
    }
}

fn select_files(client: &mut AdbCli) {
    let paths = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("APK File", &["apk"])
        .show_open_multiple_file()
        .unwrap();

    for path in paths.iter() {
        client.files.push(path.to_string_lossy().into_owned());
    }
}

pub(crate) fn controll_command(client: &mut AdbCli, cli: Adbcommand){
    match cli {
        Adbcommand::Root => exe_cmd::root_devices(client.get_devices()),
        Adbcommand::Devices => client.devices = exe_cmd::get_devices_via_adb(),
        Adbcommand::Install => {
            exe_cmd::install_apk_for_all_devices(client.get_devices(), client.get_files());
            client.clear_files();
        },
        Adbcommand::Select => select_files(client),
    }
}
