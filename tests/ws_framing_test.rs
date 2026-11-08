use serde_json::json;
use zalo_client::ws::{decode_header, encode_payload};

#[test]
fn encode_payload_has_correct_binary_format() {
    let data = json!({"hello": "world"});
    let buf = encode_payload(1, 501, 0, &data);

    assert!(buf.len() >= 4);
    assert_eq!(buf[0], 1); // version
    // cmd as little-endian u16
    assert_eq!(u16::from_le_bytes([buf[1], buf[2]]), 501);
    assert_eq!(buf[3], 0); // subCmd

    let json_bytes = serde_json::to_vec(&data).unwrap();
    assert_eq!(&buf[4..], json_bytes.as_slice());
}

#[test]
fn decode_header_parses_4_byte_header() {
    let buf = [1u8, 245, 1, 7, 9, 9]; // cmd 501 (0x01F5 LE = [245,1]), sub 7
    let (ver, cmd, sub) = decode_header(&buf).expect("should decode");
    assert_eq!(ver, 1);
    assert_eq!(cmd, 501);
    assert_eq!(sub, 7);

    assert!(decode_header(&[1, 2, 3]).is_none());
    assert!(decode_header(&[]).is_none());
}

#[test]
fn encode_decode_round_trip() {
    let cases = [
        (1u8, 501u16, 0u8),
        (1, 551, 0),
        (1, 521, 0),
        (1, 2, 1),
        (1, 3000, 0),
        (2, 65535, 255),
        (0, 0, 0),
    ];
    for (ver, cmd, sub) in cases {
        let buf = encode_payload(ver, cmd, sub, &json!({"k": 1}));
        let (dv, dc, ds) = decode_header(&buf).expect("decode should succeed");
        assert_eq!((dv, dc, ds), (ver, cmd, sub), "round-trip mismatch");
    }
}

#[test]
fn message_cmd_501_sub_0() {
    let buf = encode_payload(1, 501, 0, &json!({"msg": "hi"}));
    let (ver, cmd, sub) = decode_header(&buf).unwrap();
    assert_eq!(ver, 1);
    assert_eq!(cmd, 501);
    assert_eq!(sub, 0);
}
