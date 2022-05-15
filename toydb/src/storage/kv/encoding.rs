use crate::sql::types::Value;

pub fn encode_string(string: &str) -> Vec<u8> {
    encode_bytes(string.as_bytes())
}

/// See: https://activesphere.com/blog/2018/08/17/order-preserving-serialization
fn encode_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(0);
    encoded.extend(
        bytes
        .iter()
        .flat_map(|b| match b {
            0x00 => vec![0x00, 0xff],
            b => vec![*b],
        })
        .chain(vec![0x00, 0x00])
    );
    encoded
}

pub fn encode_value(value: &Value) -> Vec<u8> {
    match value {
        Value::Null => vec![0x00],
        Value::String(s) => [vec![0x04], encode_string(s)].concat(),
    }
}

#[cfg(test)]
mod tests {
    use super::encode_bytes;

    #[test]
    fn test_encode_bytes() {
        assert_eq!(encode_bytes(&[]), [0x00, 0x00]);
    }
}