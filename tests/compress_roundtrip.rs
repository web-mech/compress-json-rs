use serde_json::{json, Value};
use compress_json_rs::{compress, decompress};

#[test]
fn compress_decompress_roundtrip_object() {
    let value = json!({
        "a": 1,
        "b": [true, false, null],
        "c": "string",
        "d": { "nested": [1, 2, 3] }
    });
    let compressed = compress(&value);
    let decompressed = decompress(compressed);
    assert_eq!(value, decompressed);
}

#[test]
fn compress_decompress_roundtrip_array() {
    let value = json!( ["x", "y", { "z": null }] );
    let compressed = compress(&value);
    let decompressed = decompress(compressed);
    assert_eq!(value, decompressed);
}

#[test]
fn compress_decompress_roundtrip_primitives() {
    let value_str = json!("hello");
    let compressed_str = compress(&value_str);
    let decompressed_str = decompress(compressed_str);
    assert_eq!(value_str, decompressed_str);

    let value_num = json!(42.42);
    let compressed_num = compress(&value_num);
    let decompressed_num = decompress(compressed_num);
    assert_eq!(value_num, decompressed_num);

    let value_bool = json!(true);
    let compressed_bool = compress(&value_bool);
    let decompressed_bool = decompress(compressed_bool);
    assert_eq!(value_bool, decompressed_bool);

    let value_null = Value::Null;
    let compressed_null = compress(&value_null);
    let decompressed_null = decompress(compressed_null);
    assert_eq!(value_null, decompressed_null);
}