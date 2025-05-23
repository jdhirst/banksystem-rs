use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: u64,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
}

impl Customer {
    pub fn new(id: u64, name: String, address: String, phone: String, email: String) -> Self {
        Self {
            id,
            name,
            address,
            phone,
            email,
        }
    }

    pub fn update(&mut self, name: Option<String>, address: Option<String>, phone: Option<String>, email: Option<String>) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(address) = address {
            self.address = address;
        }
        if let Some(phone) = phone {
            self.phone = phone;
        }
        if let Some(email) = email {
            self.email = email;
        }
    }
}
