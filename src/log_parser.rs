use std::fs;

pub struct Message {
    // change this to set values later
    message_type: String,
    message_data: String
}

impl Message {
    pub fn parse_from_str(data: &str) -> Result<Message, String> {
        println!("{}", data);
        todo!()
    }
}

pub struct Log {
    pub data: Vec<Message>,
    pub raw_data: String
}

impl Log {
    pub fn new(file_location: &str) -> Log {
       Log {
           data: vec![],
           raw_data: fs::read_to_string(file_location).unwrap()
       }
    }

    pub fn init(&mut self) -> () {
        let log_presplit = &self.raw_data.replace("[UnityCrossThreadLogger]", "!±");
        let log_vec = log_presplit.split("!");


    }
}

