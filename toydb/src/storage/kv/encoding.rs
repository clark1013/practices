use crate::sql::types::Value;

pub fn encode_string(string: &str) -> Vec<u8> {
    encode_bytes(string.as_bytes())
}

/// See: https://activesphere.com/blog/2018/08/17/order-preserving-serialization
fn encode_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(bytes.len() + 2);
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

fn encode_boolean(b: bool) -> u8 {
    if b {
        0x01
    } else {
        0x00
    }
}

fn encode_i64(i: i64) -> [u8; 8] {
    let mut bytes = i.to_be_bytes();
    bytes[0] ^= 1 << 7; // 最高位 0->1, 1->0，便于排序
    bytes
}

fn encode_f64(f: f64) -> [u8; 8] {
    let mut bytes = f.to_be_bytes();
    if bytes[0] >> 7 & 1 == 0 {
        bytes[0] ^= 1 << 7;
    } else {
        bytes.iter_mut().for_each(| b | (*b) = !(*b));
    }
    bytes
}

pub fn encode_value(value: &Value) -> Vec<u8> {
    match value {
        Value::Null => vec![0x00],
        Value::Boolean(b) => vec![0x01, encode_boolean(*b)],
        Value::Integer(i) => [&[0x02][..], &encode_i64(*i)].concat(),
        Value::Float(f) => [&[0x03][..], &encode_f64(*f)].concat(),
        Value::String(s) => [vec![0x04], encode_string(s)].concat(),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_encode_bytes() {
        use super::encode_bytes;
        assert_eq!(encode_bytes(&[]), [0x00, 0x00]);
        assert_eq!(encode_bytes(&[0x01, 0x02, 0x03]), vec![0x01, 0x02, 0x03, 0x00, 0x00]);
        assert_eq!(encode_bytes(&[0x00, 0x01, 0x02]), vec![0x00, 0xff, 0x01, 0x02, 0x00, 0x00]);
    }

    #[test]
    fn test_encode_boolean() {
        use super::encode_boolean;
        assert_eq!(encode_boolean(true), 0x01);
        assert_eq!(encode_boolean(false), 0x00);
    }

    #[test]
    fn test_encode_i64() {
        use super::encode_i64;
        assert_eq!(encode_i64(std::i64::MIN), [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(encode_i64(0), [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(encode_i64(std::i64::MAX), [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    }

    #[test]
    fn test_encode_f64() {
        // TODO: more test case
        use super::encode_f64;
        use std::f64;
        assert_eq!(encode_f64(f64::NEG_INFINITY), [0x00, 0x0f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        assert_eq!(encode_f64(f64::INFINITY), [0xff, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }
}