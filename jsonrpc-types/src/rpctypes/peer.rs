use cita_types::Address;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub id: u32,
    pub port: u16,
    pub ip: String,
    pub proposer: Address,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Peers {
    pub peers: Vec<Peer>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn peer_serialization() {
        let value = json!({
            "id":"0"
            "port":"1339",
            "ip":"192.168.1.200",
            "proposer":"0x33990122638b9132ca29c723bdf037f1a891a70c"
        });

        let peer = Peer {
            id: 0,
            port: 1339,
            ip: "192.168.1.200".to_string(),
            proposer: Address::from_str("33990122638b9132ca29c723bdf037f1a891a70c").unwrap(),
        };

        assert_eq!(serde_json::to_value(peer).unwrap(), value);
    }
}