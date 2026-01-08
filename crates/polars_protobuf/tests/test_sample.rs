pub mod sample {
    include!(concat!(env!("OUT_DIR"), "/sample.rs"));
}

use prost::Message;

#[test]
fn test_encode_decode_empty() -> Result<(), Box<dyn std::error::Error>> {
    let message = sample::EmptyMessage {};

    let mut encoded = Vec::new();
    message.encode(&mut encoded)?;
    let decoded = sample::EmptyMessage::decode(encoded.as_slice())?;
    assert_eq!(message, decoded);
    Ok(())
}

#[test]
fn test_encode_decode_string() -> Result<(), Box<dyn std::error::Error>> {
    let message = sample::StringMessage {
        f_string: "Hello, world!".to_string(),
    };

    let mut encoded = Vec::new();
    message.encode(&mut encoded)?;
    let decoded = sample::StringMessage::decode(encoded.as_slice())?;
    assert_eq!(message, decoded);
    Ok(())
}

#[test]
fn test_encode_decode_enum() -> Result<(), Box<dyn std::error::Error>> {
    let message = sample::EnumMessage {
        f_enum: sample::Enum::Known as i32,
    };

    let mut encoded = Vec::new();
    message.encode(&mut encoded)?;
    let decoded = sample::EnumMessage::decode(encoded.as_slice())?;
    assert_eq!(message, decoded);
    Ok(())
}
