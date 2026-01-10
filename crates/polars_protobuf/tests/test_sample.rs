use polars_protobuf::ArrowMessage;

pub mod sample {
    include!(concat!(env!("OUT_DIR"), "/tests/sample.rs"));
}

use prost::Message;

macro_rules! test_encode_decode {
    ($message:ident, $prost_message:expr, $arrow_message:expr) => {
        assert_eq!(
            sample::$message::from_prost($prost_message.clone()),
            $arrow_message
        );
        assert_eq!($arrow_message.clone().to_prost(), $prost_message);

        let prost_encoded = $prost_message.encode_to_vec();
        let prost_decoded = sample::prost::$message::decode(prost_encoded.as_slice())?;
        assert_eq!($prost_message, prost_decoded);

        let arrow_encoded = $arrow_message.encode_to_vec();
        let arrow_decoded = sample::$message::decode(arrow_encoded.as_slice())?;
        assert_eq!($arrow_message, arrow_decoded);
    };
}

#[test]
fn test_encode_decode_empty() -> Result<(), Box<dyn std::error::Error>> {
    let prost_message = sample::prost::EmptyMessage {};
    let arrow_message = sample::EmptyMessage {};
    test_encode_decode!(EmptyMessage, prost_message, arrow_message);
    Ok(())
}

#[test]
fn test_encode_decode_string() -> Result<(), Box<dyn std::error::Error>> {
    let prost_message = sample::prost::StringMessage {
        f_string: "Hello, world!".to_string(),
    };
    let arrow_message = sample::StringMessage {
        f_string: "Hello, world!".to_string(),
    };
    test_encode_decode!(StringMessage, prost_message, arrow_message);

    Ok(())
}

#[test]
fn test_encode_decode_enum() -> Result<(), Box<dyn std::error::Error>> {
    let prost_message = sample::prost::EnumMessage {
        f_enum: sample::prost::Enum::Known as i32,
    };
    let arrow_message = sample::EnumMessage {
        f_enum: sample::Enum::KNOWN,
    };
    test_encode_decode!(EnumMessage, prost_message, arrow_message);

    Ok(())
}
