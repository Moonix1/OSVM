#[derive(Clone)]
pub struct Label {
    pub name: String,
    pub addr: usize,
}

#[derive(Clone)]
pub struct DeferredOperand {
    pub addr: usize,
    pub label: String,
}

pub struct OASM {
    pub labels: Vec<Label>,
    pub deferred_operands: Vec<DeferredOperand>,
}

impl OASM {
    pub fn init() -> OASM {
        OASM {
            labels: Vec::new(),
            deferred_operands: Vec::new(),
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
    
    pub fn deferred_operands_push(self: &mut Self, label_name: &str, jump_addr: usize) {
        self.deferred_operands.push(DeferredOperand {
            addr: jump_addr,
            label: label_name.to_string()
        });
    }
}