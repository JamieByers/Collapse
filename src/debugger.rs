pub struct Debugger {
    pub enabled: bool,
    pub logs : Vec<String>,
}

impl Debugger {
    pub fn new(enabled: bool) -> Self {
        Debugger {
            enabled,
            logs: Vec::new()
        }
    }

    pub fn log(&mut self, message: String) {
        if self.enabled {
            self.logs.push(message);
        }
    }

    pub fn display_logs(&self) {
        if self.enabled {
            println!("--- Debug Logs ---");
            println!("{:?}", self.logs);
            for log in &self.logs {
                println!("[DEBUG] {}", log);
            }
        }
    }
}







