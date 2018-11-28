use rpctypes::{Data, Data20, Data32};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct EstimateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Data20>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Data20>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Data32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

impl EstimateRequest {
    pub fn new(from: Option<Data20>, to: Option<Data20>, value: Option<Data32>, data: Option<Data>) -> Self {
        EstimateRequest { from, to, value, data }
    }
}

#[cfg(test)]
mod tests {
    use super::EstimateRequest;
    use cita_types::{ H160, H256 };
    use serde_json;

    #[test]
    fn deserialize() {
        let testdata = vec![(
            json!({
                "from": "0x0000000000000000000000000000000000000001",
                "to": "0x0000000000000000000000000000000000000002",
                "value" : "";
                "data": "0xabcdef"
            }).to_string(),
            Some(EstimateRequest::new(
                Some(H160::from(1).into()),
                Some(H160::from(2).into()),
                Some(H256::form(3).into()),
                Some(vec![0xab, 0xcd, 0xef].into()),
            )),
        )];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<EstimateRequest, serde_json::Error> = serde_json::from_str(&data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), expected);
            } else {
                assert!(result.is_err());
            }
        }
    }
}