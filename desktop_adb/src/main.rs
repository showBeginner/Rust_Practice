mod adb_lib {
    pub(crate) mod adb_struct;
}
use crate::adb_lib::adb_struct::{AdbCli, controll_command, Adbcommand};
use iced::executor;
use iced::{Application, Command, Element, Settings, Theme, Length};
use iced::widget::{column, row, button, container, text};

fn main() -> iced::Result {
    Desktop::run(Settings {
        default_text_size: 20 as f32,
        ..Settings::default()
    })
}

struct Desktop {
    adb_client: AdbCli,
}

#[derive(Debug, Clone)]
enum Message {
    Reloaded,
    Install,
    Select,
}


impl Application for Desktop {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags:()) -> (Desktop, Command<Self::Message>) {
        (Desktop {adb_client: AdbCli::new()}, Command::none())
    }


    fn title(&self) -> String {
        String::from("ADB Desktop")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Reloaded => {
                controll_command(&mut self.adb_client, Adbcommand::Devices);
                controll_command(&mut self.adb_client, Adbcommand::Root);
                println!("Devices: {:?}", self.adb_client.get_devices());
            }
            Message::Install => {
                controll_command(&mut self.adb_client, Adbcommand::Install);
            }
            Message::Select => {
                controll_command(&mut self.adb_client, Adbcommand::Select)
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let device_list = self.adb_client.get_devices()
            .iter()
            .fold(
                column![text("Devices list:")].spacing(10),
                |column, device| {
                    column.push(text(device))
                },
            );
        let files_list = self.adb_client.get_files()
            .iter()
            .fold(
                column![text("Files list:")].spacing(10),
                |column, file| {
                    column.push(text(file))
                },
            );
        let button_tr = row![
            button("Reload").on_press(Message::Reloaded),
            button("Select").on_press(Message::Select),
            button("Install").on_press(Message::Install),
        ]
        .width(300)
        .height(100)
        .spacing(10);
        let content = column![
            device_list,
            files_list,
            button_tr,
        ]
        .spacing(20)
        .padding(20)
        .max_width(800);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
