use rpc_types::{Data, Data20, Quantity};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct EstimateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Data20>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Data20>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

impl EstimateRequest {
    pub fn new(
        from: Option<Data20>,
        to: Option<Data20>,
        value: Option<Quantity>,
        data: Option<Data>,
    ) -> Self {
        EstimateRequest {
            from,
            to,
            value,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EstimateRequest;
    use cita_types::{H160, U256};
    use serde_json;

    #[test]
    fn deserialize() {
        let testdata = vec![(
            json!({
                "from": "0x0000000000000000000000000000000000000001",
                "to": "0x0000000000000000000000000000000000000002",
                "value" : "0xabcdef123",
                "data": "0xabcdef"
            })
            .to_string(),
            Some(EstimateRequest::new(
                Some(H160::from(1).into()),
                Some(H160::from(2).into()),
                Some(U256::from(0xabcdef123u64).into()),
                Some(vec![0xab, 0xcd, 0xef].into()),
            )),
        )];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<EstimateRequest, serde_json::Error> = serde_json::from_str(&data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.ok().unwrap(), expected)
            } else {
                assert!(false)
            }
        }
    }
}
