use mac_address::MacAddress;

pub struct User {
    pub id: u64,
    pub mac_addr: MacAddress,
    pub device_name: String,
    pub nickname: String,
    pub status: String,
}

impl User {
    pub fn to_string(&self) -> String {
        format!("{}, {}, {}", self.mac_addr, self.device_name, self.nickname)
    }

    pub fn constructor(
        id: &u64,
        mac: &str,
        device_name: &str,
        nickname: &str,
        status: &str
        ) -> Option<User> {
        match mac.parse::<MacAddress>() {
            Ok(mac_addr) => Some(User {
                id: *id,
                mac_addr,
                device_name: device_name.to_string(),
                nickname: nickname.to_string(),
                status: status.to_string(),
            }),
            Err(_) => None,
        }
    }
}
