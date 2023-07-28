use tui::widgets::ListState;



pub(crate) enum InputMode {
    Normal,
    Editing,
}

pub(crate) struct StatefulList<T> {
    pub(crate) state: ListState,
    pub(crate) items: Vec<T>,
}


pub(crate) struct App<'a> {
    input: String,
    input_mode: InputMode,
    show_popup: bool,
    command: String,
    pub(crate) items: StatefulList<&'a str>,
}


impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub(crate) fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub(crate) fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub(crate) fn get_index(self) -> Option<usize> {
        if let Some(i) = self.state.selected() {
            return Some(i);
        }
        None
    }

    pub(crate) fn set_items(&mut self, i: usize, value: T) {
        self.items[i] = value;
    }

    pub(crate) fn push_items(&mut self, value: T) {
        self.items.push(value);
    }

    pub(crate) fn clear_items(&mut self) {
        self.items.clear();
    }

    pub(crate) fn unselect(&mut self) {
        self.state.select(None);
    }
}


impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            show_popup:false,
            command: String::new(),
            items: StatefulList::with_items(vec![]),
        }
    }
    
}

impl App <'_>{

    pub(crate) fn get_input(&self) -> &String {
        &self.input
    }

    pub(crate) fn get_command(&self) -> &String {
        &self.command
    }

    pub(crate) fn command_clear(&mut self){
        &self.command.clear();
    }
    pub(crate) fn command_push(&mut self, value:char){
        &self.command.push(value);
    }
    pub(crate) fn command_pop(&mut self){
        &self.command.pop();
    }

    pub(crate) fn get_input_mode(&self) -> &InputMode {
        &self.input_mode
    }
    pub(crate) fn set_input_mode(&mut self,value: InputMode){
        self.input_mode = value;
    }

    pub(crate) fn get_popup<'a> (&'a self) -> &'a bool {
        &self.show_popup
    }
    pub(crate) fn set_popup(&mut self,value: bool) {
        self.show_popup = value;
    }
}

