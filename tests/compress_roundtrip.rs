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

    let value_num = json!(916);
    let compressed_int = compress(&value_num);
    let decompressed_int = decompress(compressed_int);
    assert_eq!(value_num, decompressed_int);

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

#[test]
fn decompress_specific_case() {
    let compressed_str = r#"[
     [
       "id",
       "isActive",
       "isTrial",
       "expirationDate",
       "trialExpirationDate",
       "product",
       "a|0|1|2|3|4|5",
       "2c1de054-491d-46ba-9c4b-ab45f2ad0003",
       "b|F",
       "2025-12-31T23:59:59.999Z",
       "productGroupName",
       "productName",
       "slug",
       "a|A|B|C",
       "Rust",
       "rust",
       "o|D||E|F",
       "o|6|7|8|8|9||G",
       "a61fe5a5-752a-4115-ae7a-76722514b3cc",
       "b|T",
       "2099-01-01T23:59:59.000Z",
       "2025-01-31T23:59:59.999Z",
       "C++",
       "c++",
       "o|D||M|N",
       "o|6|I|J|8|K|L|O",
       "d6a695cc-6d4d-44d5-9537-06b4e6bc3e0d",
       "2099-01-01T23:59:59.999Z",
       "API",
       "api",
       "o|D||S|T",
       "o|6|Q|J|8|R||U",
       "a6dcf066-e7ed-41dd-8f66-6389a80c58e1",
       "API",
       "api",
       "o|D||X|Y",
       "o|6|W|J|8|R|R|Z",
       "deeaa984-ad5e-45d4-99f4-5a88cc1c79de",
       "API2",
       "api2",
       "o|D||c|d",
       "o|6|b|J|8|R|R|e",
       "f4eff35c-4615-4ab3-a383-27684a3b4cb1",
       "API3",
       "api3",
       "o|D||h|i",
       "o|6|g|J|8|R|R|j",
       "d1873eac-5650-43cb-818d-8a28d8f54f63",
       "API4",
       "api4",
       "o|D||m|n",
       "o|6|l|J|8|R|R|o",
       "a|H|P|V|a|f|k|p"
     ],
     "q"
   ]"#;

    // 1. Parse the JSON string representation of the compressed data
    let parsed_value: Value = serde_json::from_str(compressed_str)
        .expect("Failed to parse compressed JSON string representation");

    // 2. Convert the parsed Value into the Compressed tuple format (Vec<String>, String)
    let compressed_tuple: (Vec<String>, String) = match parsed_value {
        Value::Array(mut outer_vec) if outer_vec.len() == 2 => {
            let key_id_val = outer_vec.pop().expect("Outer array missing second element");
            let values_val = outer_vec.pop().expect("Outer array missing first element");

            let values: Vec<String> = match values_val {
                Value::Array(inner_vec) => inner_vec
                    .into_iter()
                    .map(|v| match v {
                        Value::String(s) => s,
                        _ => panic!("Inner array element is not a string: {:?}", v),
                    })
                    .collect(),
                _ => panic!("First element is not an array: {:?}", values_val),
            };

            let key_id: String = match key_id_val {
                Value::String(s) => s,
                _ => panic!("Second element is not a string: {:?}", key_id_val),
            };

            (values, key_id)
        }
        _ => panic!("Parsed value is not a two-element array: {:?}", parsed_value),
    };

    // 3. Decompress the value using the tuple
    let decompressed = decompress(compressed_tuple);

    // 4. Define the expected decompressed value
    let expected_value = json!( [
      {
        "expirationDate": "2025-12-31T23:59:59.999Z",
        "id": "2c1de054-491d-46ba-9c4b-ab45f2ad0003",
        "isActive": false,
        "isTrial": false,
        "product": {
          "productGroupName": null,
          "productName": "Rust",
          "slug": "rust"
        },
        "trialExpirationDate": null
      },
      {
        "expirationDate": "2099-01-01T23:59:59.000Z",
        "id": "a61fe5a5-752a-4115-ae7a-76722514b3cc",
        "isActive": true,
        "isTrial": false,
        "product": {
          "productGroupName": null,
          "productName": "C++",
          "slug": "c++"
        },
        "trialExpirationDate": "2025-01-31T23:59:59.999Z"
      },
      {
        "expirationDate": "2099-01-01T23:59:59.999Z",
        "id": "d6a695cc-6d4d-44d5-9537-06b4e6bc3e0d",
        "isActive": true,
        "isTrial": false,
        "product": {
          "productGroupName": null,
          "productName": "API",
          "slug": "api"
        },
        "trialExpirationDate": null
      },
      {
        "expirationDate": "2099-01-01T23:59:59.999Z",
        "id": "a6dcf066-e7ed-41dd-8f66-6389a80c58e1",
        "isActive": true,
        "isTrial": false,
        "product": {
          "productGroupName": null,
          "productName": "API",
          "slug": "api"
        },
        "trialExpirationDate": "2099-01-01T23:59:59.999Z"
      },
      {
        "expirationDate": "2099-01-01T23:59:59.999Z",
        "id": "deeaa984-ad5e-45d4-99f4-5a88cc1c79de",
        "isActive": true,
        "isTrial": false,
        "product": {
          "productGroupName": null,
          "productName": "API2",
          "slug": "api2"
        },
        "trialExpirationDate": "2099-01-01T23:59:59.999Z"
      },
      {
        "expirationDate": "2099-01-01T23:59:59.999Z",
        "id": "f4eff35c-4615-4ab3-a383-27684a3b4cb1",
        "isActive": true,
        "isTrial": false,
        "product": {
          "productGroupName": null,
          "productName": "API3",
          "slug": "api3"
        },
        "trialExpirationDate": "2099-01-01T23:59:59.999Z"
      },
      {
        "expirationDate": "2099-01-01T23:59:59.999Z",
        "id": "d1873eac-5650-43cb-818d-8a28d8f54f63",
        "isActive": true,
        "isTrial": false,
        "product": {
          "productGroupName": null,
          "productName": "API4",
          "slug": "api4"
        },
        "trialExpirationDate": "2099-01-01T23:59:59.999Z"
      }
    ]);

    // 5. Assert equality
    assert_eq!(expected_value, decompressed);
}