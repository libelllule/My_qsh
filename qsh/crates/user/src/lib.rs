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
        id: u64,
        mac: String,
        device_name: String,
        nickname: String,
        status: String
        ) -> Option<User> {
        match mac.parse::<MacAddress>() {
            Ok(mac_addr) => Some(User {
                id,
                mac_addr,
                device_name,
                nickname,
                status,
            }),
            Err(_) => None,
        }
    }
}
