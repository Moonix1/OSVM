use crate::defines::Word;

#[derive(Clone)]
pub struct Label {
    pub name: String,
    pub addr: usize,
}

#[derive(Clone)]
pub struct Jumps {
    pub addr: usize,
    pub label: String,
}

pub struct OASM {
    pub labels: Vec<Label>,
    pub jumps: Vec<Jumps>,
}

impl OASM {
    pub fn init() -> OASM {
        OASM {
            labels: Vec::new(),
            jumps: Vec::new(),
        }
    }
    
    pub fn labels_contains(self: &Self, label_name: &str) -> Option<i64> {
        for i in 0..self.labels.len() {
            if self.labels[i].name == label_name {
                return Some(self.labels[i].addr as i64);
            }
        }
        
        eprintln!("[Error]: label: `{}` does not exist!", label_name);
        None
    }
    
    pub fn labels_push(self: &mut Self, label_name: &str, label_addr: usize) {
        self.labels.push(Label {
            name: label_name.to_string(),
            addr: label_addr
        });
    }
    
    pub fn jumps_push(self: &mut Self, label_name: &str, jump_addr: usize) {
        self.jumps.push(Jumps {
            addr: jump_addr,
            label: label_name.to_string()
        });
    }
}