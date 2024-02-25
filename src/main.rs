use std::{collections::HashSet, sync::Arc};

use iced::{Sandbox, Element, widget::TextInput};

#[derive(Debug, Clone, Copy)]
pub struct ConfigFileIndex(usize);

#[derive(Debug, Clone, Copy)]
pub struct ConfigVariableIndex(usize);


#[derive(Debug, Clone)]
pub enum Message {
    NewFileLoadRequested{
        file_path: String
    },
    ExistingFileSaveRequested {
        /// Index into the resident vector of config files, that can be used to identify the
        file: ConfigFileIndex,
    },
    ChangeOfVariable {
        file: ConfigFileIndex,
        variable: ConfigVariableIndex,
    },

}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum KnownFormat {
    #[default]
    Json,
    Toml, 
    Yaml, 
}

#[derive(Debug, Clone)]
pub struct PermittedValues {
    all_acceptable_values: HashSet<Box<str>>,
}


#[derive(Debug, Clone)]
pub struct PermittedCharacters {
    all_acceptable_characters: HashSet<char>,
}

#[derive(Debug, Clone)]
pub enum Variable {
    Integer {
        range: [u128; 2],
        value: u128
    },
    Float {
        range: [f64; 2],
        value: f64
    },
    DecimalFraction {
        range: [f64; 2],
        value: f64 // FIXME: Use decimal 
    },
    String {
        permitted_values: Arc<PermittedCharacters>,
        value: String,
    },
    Enumeration {
        permitted_values: Arc<PermittedValues>,
        value: String,
    }, 
    Group {
        values: Vec<Variable>,
    }
}

#[derive(Clone, Debug)]
pub struct ConfigFile {
    path: std::path::PathBuf,
    format: KnownFormat,

    variables: Vec<Variable>
}

#[derive(Debug, Clone, Default)]
pub struct ConfigUi {
    config_files: Vec<ConfigFile>
}

impl Sandbox for ConfigUi {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Hello, World!")
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    fn view(&self) -> Element<Self::Message> {
        todo!()
    }
}

fn main() {
    let _ = ConfigUi::run(iced::Settings::default());
}
