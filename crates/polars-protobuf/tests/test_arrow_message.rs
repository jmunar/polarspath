use polars_core::prelude::*;
use polars_lazy::prelude::*;
use polars_protobuf::{decode_expr, encode_expr, messages_to_series, ArrowMessage};
use polars_structpath::FromArrow;
use prost::Message;

pub mod sample {
    include!(concat!(env!("OUT_DIR"), "/tests/sample.rs"));
}

macro_rules! test_encode_decode {
    ($message:ident, $prost_msg:expr, $arrow_msg:expr) => {
        // arrow == prost->arrow
        assert_eq!(sample::$message::from_prost($prost_msg.clone()), $arrow_msg);

        // prost == arrow->prost
        assert_eq!($arrow_msg.clone().to_prost(), $prost_msg);

        // prost == prost encoding + decoding
        let prost_encoded = $prost_msg.encode_to_vec();
        let prost_decoded = sample::prost::$message::decode(prost_encoded.as_slice())?;
        assert_eq!($prost_msg, prost_decoded);

        // arrow == arrow encoding + decoding
        let arrow_encoded = $arrow_msg.encode_to_vec();
        let arrow_decoded = sample::$message::decode(arrow_encoded.as_slice())?;
        assert_eq!($arrow_msg, arrow_decoded);

        // prost encoding == arrow encoding (we only encode the prost version!)
        assert_eq!(prost_encoded, arrow_encoded);

        // Test lazy API roundtrip: struct -> bytes -> struct
        // Use 2 messages to avoid Polars ScalarColumn optimization issue with List<UInt8>
        let messages = vec![$arrow_msg.clone(), $arrow_msg.clone()];
        let series = messages_to_series(messages, "test")?;
        let struct_dtype = series.dtype().clone();
        let df = DataFrame::new(vec![series.into()])?;

        // Encode using lazy API
        let encoded_df = df
            .clone()
            .lazy()
            .select([encode_expr::<sample::$message>(col("test")).alias("encoded")])
            .collect()?;

        // Decode using lazy API
        let decoded_df = encoded_df
            .lazy()
            .select([
                decode_expr::<sample::$message>(col("encoded"), struct_dtype).alias("decoded"),
            ])
            .collect()?;

        // Verify roundtrip
        let original_chunks = df
            .column("test")?
            .as_materialized_series()
            .clone()
            .into_chunks();
        let decoded_chunks = decoded_df
            .column("decoded")?
            .as_materialized_series()
            .clone()
            .into_chunks();

        let original_messages = sample::$message::from_arrow(original_chunks[0].clone());
        let decoded_messages = sample::$message::from_arrow(decoded_chunks[0].clone());

        assert_eq!(original_messages, decoded_messages);
    };
}

#[test]
fn test_encode_decode_empty() -> Result<(), Box<dyn std::error::Error>> {
    let prost_msg = sample::prost::EmptyMessage {};
    let arrow_msg = sample::EmptyMessage {};
    test_encode_decode!(EmptyMessage, prost_msg, arrow_msg);
    Ok(())
}

#[test]
fn test_encode_decode_string() -> Result<(), Box<dyn std::error::Error>> {
    let prost_msg = sample::prost::StringMessage {
        f_string: "Hello, world!".to_string(),
    };
    let arrow_msg = sample::StringMessage {
        f_string: "Hello, world!".to_string(),
    };
    test_encode_decode!(StringMessage, prost_msg, arrow_msg);

    Ok(())
}

#[test]
fn test_encode_decode_enum() -> Result<(), Box<dyn std::error::Error>> {
    let prost_msg = sample::prost::EnumMessage {
        f_enum: sample::prost::Enum::Known as i32,
    };
    let arrow_msg = sample::EnumMessage {
        f_enum: sample::Enum::KNOWN,
    };
    test_encode_decode!(EnumMessage, prost_msg, arrow_msg);

    Ok(())
}
