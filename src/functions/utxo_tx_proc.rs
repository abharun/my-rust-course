use std::collections::HashMap;

pub enum InputType {
    Deposit(String, u64),
    Transfer(String, String, u64),
    Display,
}

pub struct Mempool {
    utxo: HashMap<String, Vec<u64>>,
}

impl Default for Mempool {
    fn default() -> Self {
        Self {
            utxo: HashMap::new(),
        }
    }
}

trait ProcessTx {
    fn is_available(&self, addr: String, amount: u64) -> bool;
    fn insert_utxo_pool(&mut self, addr: String, amount: u64);
    fn update_utxo_pool(&mut self, sender: String, receiver: String, amount: u64);
}

impl ProcessTx for Mempool {
    fn is_available(&self, addr: String, amount: u64) -> bool {
        match self.utxo.get(&addr) {
            Some(utxo) => {
                let sum = utxo.iter().sum::<u64>();
                sum >= amount
            }
            None => false,
        }
    }

    fn insert_utxo_pool(&mut self, addr: String, amount: u64) {
        match self.utxo.get_mut(&addr) {
            Some(addr_utxo) => {
                addr_utxo.push(amount);
            }
            None => {
                self.utxo.insert(addr, vec![amount]);
            }
        }
    }

    fn update_utxo_pool(&mut self, sender: String, receiver: String, amount: u64) {
        let mut resvalue = amount;
        if !self.is_available(sender.clone(), amount) {
            println!("Failed to transfer!");
            return;
        }
        let mut sender_utxo = self.utxo.get_mut(&sender).unwrap().clone();
        let receiver_utxo = self.utxo.get_mut(&receiver).unwrap();
        while resvalue > 0 {
            let value = sender_utxo[0];
            if resvalue >= value {
                resvalue -= value;
                sender_utxo.remove(0);
            } else {
                sender_utxo.push(value - resvalue);
                sender_utxo.remove(0);
                resvalue = 0;
            }
        }
        receiver_utxo.push(amount);
        self.utxo.insert(sender, sender_utxo);
    }
}

pub trait Interface {
    fn run_cli(&mut self, command: String);
    fn parse_cli(command: String) -> InputType;
}

impl Interface for Mempool {
    fn run_cli(&mut self, command: String) {
        let input = Self::parse_cli(command);
        match input {
            InputType::Deposit(addr, amount) => {
                self.insert_utxo_pool(addr, amount);
            }
            InputType::Transfer(sender, receiver, amount) => {
                self.update_utxo_pool(sender, receiver, amount);
            }
            InputType::Display => {
                for (addr, utxos) in &self.utxo {
                    println!("{:?}\n{:?}", addr, utxos);
                }
            }
        };
    }

    fn parse_cli(command: String) -> InputType {
        let text: Vec<&str> = command.split_whitespace().map(|cmd| cmd).collect();
        if text.len() == 3 {
            let (sender, receiver, amount) = (
                text[0].to_string(),
                text[1].to_string(),
                text[2].parse::<u64>().unwrap(),
            );
            InputType::Transfer(sender, receiver, amount)
        } else if text.len() == 2 {
            let (addr, amount) = (text[0].to_string(), text[1].parse::<u64>().unwrap());
            InputType::Deposit(addr, amount)
        } else {
            InputType::Display
        }
    }
}
