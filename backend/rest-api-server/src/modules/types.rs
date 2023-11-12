use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sensor {
    pub name: Option<String>,
    pub unit: Option<String>
}

impl Sensor {
    pub fn validate(self) -> Option<Register> {
        match (&self.name, &self.unit) {
            (Some(_), Some(_)) => Some(self),
            _ => None
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Data {
    pub timestamp: Option<String>, // Optional
    pub values: Option<String>,
    pub device_uuid: Option<String>,
    pub sensor: Option<String>
}

impl Data {
    pub fn validate(self) -> Option<Register> {
        match (&self.values, &self.device_uuid, &self.sensor) {
            (Some(_), Some(_), Some(_)) => Some(self),
            _ => None
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Devices {
    pub id: Option<String>,
    pub position: Option<String>,
    pub address: Option<String>,
    pub is_vehicle: Option<bool>
}

impl Devices {
    pub fn validate(self) -> Option<Register> {
        match (&self.id, &self.posistion, &self.address, &self.is_vehicle) {
            (Some(_), Some(_), Some(_), Some(_)) => Some(self),
            _ => None
        }
    }
}
