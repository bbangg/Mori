use crate::utils::{
    proton::hash_string,
    random::{random_hex, random_mac_address},
};

#[derive(Debug)]
pub struct LoginInfo {
    pub uuid: String,
    pub protocol: String,
    pub fhash: String,
    pub mac: String,
    pub requested_name: String,
    pub hash2: String,
    pub fz: String,
    pub f: String,
    pub player_age: String,
    pub game_version: String,
    pub lmode: String,
    pub cbits: String,
    pub rid: String,
    pub gdpr: String,
    pub hash: String,
    pub category: String,
    pub token: String,
    pub total_playtime: String,
    pub door_id: String,
    pub klv: String,
    pub meta: String,
    pub platform_id: String,
    pub device_version: String,
    pub zf: String,
    pub country: String,
    pub user: String,
    pub wk: String,
}

impl LoginInfo {
    pub fn new() -> LoginInfo {
        LoginInfo {
            uuid: String::new(),
            protocol: "209".to_string(),
            fhash: "-716928004".to_string(),
            mac: random_mac_address(),
            requested_name: "BraveDuck".to_string(),
            hash2: String::new(),
            fz: "47142936".to_string(),
            f: "1".to_string(),
            player_age: "20".to_string(),
            game_version: "4.62".to_string(),
            lmode: "1".to_string(),
            cbits: "1040".to_string(),
            rid: random_hex(32, true),
            gdpr: "3".to_string(),
            hash: "0".to_string(),
            category: "_-5100".to_string(),
            token: String::new(),
            total_playtime: "0".to_string(),
            door_id: String::new(),
            klv: String::new(),
            meta: String::new(),
            platform_id: "0,1,1".to_string(),
            device_version: "0".to_string(),
            zf: "-821693372".to_string(),
            country: "us".to_string(),
            user: String::new(),
            wk: random_hex(32, true),
        }
    }
}