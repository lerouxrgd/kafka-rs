mod compression;
mod crc32;

pub mod de;
pub mod error;
pub mod ser;

pub use crate::codec::compression::Compression;
pub use crate::codec::de::{decode_resp, read_resp, Deserializer};
pub use crate::codec::error::{Error, Result};
pub use crate::codec::ser::{encode_req, Serializer};

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use std::io::Cursor;

    use matches::assert_matches;

    use super::*;
    use crate::codec::{de::zag_i32, ser::zig_i32};
    use crate::model::*;
    use crate::types::*;

    fn encode_single<T: Serialize>(val: &T) -> Result<Vec<u8>> {
        let mut serializer = Serializer::new();
        val.serialize(&mut serializer)?;
        Ok(serializer.buf[4..].to_vec())
    }

    fn decode_single<'a, T>(input: &'a [u8], version: Option<usize>) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        let mut deserializer = Deserializer::from_bytes(input, version.unwrap_or_else(|| 0));
        let resp = T::deserialize(&mut deserializer)?;

        if deserializer.len() == 0 {
            Ok(resp)
        } else {
            Err(serde::de::Error::custom(format!("bytes remaining",)))
        }
    }

    #[test]
    fn serde_bool() {
        let v1 = true;
        let bytes = encode_single(&v1).unwrap();
        let v2 = decode_single::<bool>(&bytes, None).unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn serde_integers() {
        let v1 = 13 as i8;
        let bytes = encode_single(&v1).unwrap();
        let v2 = decode_single::<i8>(&bytes, None).unwrap();
        assert_eq!(v1, v2);

        let v1 = 13 as i16;
        let bytes = encode_single(&v1).unwrap();
        let v2 = decode_single::<i16>(&bytes, None).unwrap();
        assert_eq!(v1, v2);

        let v1 = 13 as i32;
        let bytes = encode_single(&v1).unwrap();
        let v2 = decode_single::<i32>(&bytes, None).unwrap();
        assert_eq!(v1, v2);

        let v1 = 13 as i64;
        let bytes = encode_single(&v1).unwrap();
        let v2 = decode_single::<i64>(&bytes, None).unwrap();
        assert_eq!(v1, v2);

        let v1 = 13 as u32;
        let bytes = encode_single(&v1).unwrap();
        let v2 = decode_single::<u32>(&bytes, None).unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn serde_varint_varlong() {
        let i: i32 = 3;
        let mut bytes = vec![];
        zig_i32(i, &mut bytes).unwrap();
        let mut rdr = Cursor::new(bytes);
        let (j, varint_size) = zag_i32(&mut rdr).unwrap();
        assert_eq!(i, j);
        assert_eq!(1, varint_size);

        let i = Varint(3);
        let bytes = encode_single(&i).unwrap();
        let j = decode_single::<Varint>(&bytes, None).unwrap();
        assert_eq!(i, j);

        let i = Varlong(-3);
        let bytes = encode_single(&i).unwrap();
        let j = decode_single::<Varlong>(&bytes, None).unwrap();
        assert_eq!(i, j);

        let i = Varint::size_of(12121662);
        let j = encode_single(&Varint(12121662)).unwrap();
        assert_eq!(i, j.len());
    }

    #[test]
    fn serde_strings() {
        let s1 = String::from("yes");
        let bytes = encode_single(&s1).unwrap();
        let s2 = decode_single::<String>(&bytes, None).unwrap();
        assert_eq!(s1, s2);

        let s1 = NullableString::from("yes");
        let bytes = encode_single(&s1).unwrap();
        let s2 = decode_single::<NullableString>(&bytes, None).unwrap();
        assert_eq!(s1, s2);

        let s1 = NullableString(None);
        let bytes = encode_single(&s1).unwrap();
        let s2 = decode_single::<NullableString>(&bytes, None).unwrap();
        assert_eq!(s1, s2);
    }

    #[test]
    fn serde_bytes() {
        let b1 = Bytes(vec![1, 2, 3]);
        let bytes = encode_single(&b1).unwrap();
        let b2 = decode_single::<Bytes>(&bytes, None).unwrap();
        assert_eq!(b1, b2);

        let b1 = NullableBytes::from(vec![1, 2, 3]);
        let bytes = encode_single(&b1).unwrap();
        let b2 = decode_single::<NullableBytes>(&bytes, None).unwrap();
        assert_eq!(b1, b2);

        let b1 = NullableBytes(None);
        let bytes = encode_single(&b1).unwrap();
        let b2 = decode_single::<NullableBytes>(&bytes, None).unwrap();
        assert_eq!(b1, b2);
    }

    #[test]
    fn versions_req_ser() {
        let header = HeaderRequest {
            api_key: ApiKey::ApiVersions,
            api_version: 0,
            correlation_id: 42,
            client_id: NullableString(None),
        };
        let bytes = encode_req(&header, &ApiVersionsRequest::V0 {}).unwrap();
        assert_eq!(vec![0, 0, 0, 10, 0, 18, 0, 0, 0, 0, 0, 42, 255, 255], bytes);
    }

    #[test]
    fn versions_resp_de() {
        let mut bytes = Cursor::new(vec![
            0, 0, 1, 12, 0, 0, 0, 42, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0, 0, 7, 0, 1, 0, 0, 0, 10, 0, 2,
            0, 0, 0, 4, 0, 3, 0, 0, 0, 7, 0, 4, 0, 0, 0, 1, 0, 5, 0, 0, 0, 0, 0, 6, 0, 0, 0, 4, 0,
            7, 0, 0, 0, 1, 0, 8, 0, 0, 0, 6, 0, 9, 0, 0, 0, 5, 0, 10, 0, 0, 0, 2, 0, 11, 0, 0, 0,
            3, 0, 12, 0, 0, 0, 2, 0, 13, 0, 0, 0, 2, 0, 14, 0, 0, 0, 2, 0, 15, 0, 0, 0, 2, 0, 16,
            0, 0, 0, 2, 0, 17, 0, 0, 0, 1, 0, 18, 0, 0, 0, 2, 0, 19, 0, 0, 0, 3, 0, 20, 0, 0, 0, 3,
            0, 21, 0, 0, 0, 1, 0, 22, 0, 0, 0, 1, 0, 23, 0, 0, 0, 2, 0, 24, 0, 0, 0, 1, 0, 25, 0,
            0, 0, 1, 0, 26, 0, 0, 0, 1, 0, 27, 0, 0, 0, 0, 0, 28, 0, 0, 0, 2, 0, 29, 0, 0, 0, 1, 0,
            30, 0, 0, 0, 1, 0, 31, 0, 0, 0, 1, 0, 32, 0, 0, 0, 2, 0, 33, 0, 0, 0, 1, 0, 34, 0, 0,
            0, 1, 0, 35, 0, 0, 0, 1, 0, 36, 0, 0, 0, 0, 0, 37, 0, 0, 0, 1, 0, 38, 0, 0, 0, 1, 0,
            39, 0, 0, 0, 1, 0, 40, 0, 0, 0, 1, 0, 41, 0, 0, 0, 1, 0, 42, 0, 0, 0, 1,
        ]);

        let (header, resp) = read_resp::<_, ApiVersionsResponse>(&mut bytes, 0).unwrap();
        assert_eq!(42, header.correlation);
        assert_matches!(resp, ApiVersionsResponse::V0 {
            error_code,ref  api_versions
        } if error_code == 0 && api_versions.len() == 43);
    }

    #[test]
    fn topics_req_resp_serde() {
        let val1 = CreateTopicsRequest::V0 {
            topics: vec![create_topics_request::v0::Topics {
                name: "topic".to_owned(),
                num_partitions: 32,
                replication_factor: 16,
                assignments: vec![create_topics_request::v0::Assignments {
                    partition_index: 12,
                    broker_ids: vec![1],
                }],
                configs: vec![create_topics_request::v0::Configs {
                    name: "default".to_owned(),
                    value: NullableString(None),
                }],
            }],
            timeout_ms: 0,
        };

        let bytes = encode_single(&val1).unwrap();
        let val2 = decode_single::<CreateTopicsRequest>(&bytes, Some(0)).unwrap();
        assert_eq!(val1, val2);
    }

    #[test]
    fn fetch_resp_de() {
        let mut bytes = vec![
            0, 0, 0, 42, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 224, 87, 178, 51,
            0, 0, 255, 255, 255, 255, 0, 0, 0, 6, 99, 111, 117, 99, 111, 117, 255, 255, 255, 255,
            255, 255, 255, 255, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let (header, resp) = decode_resp::<FetchResponse>(&mut bytes, 0).unwrap();
        println!("{:?}", header);
        println!("{:?}", resp);
    }
}
