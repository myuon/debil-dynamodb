# debil-dynamodb  [![debil-dynamodb at crates.io](https://img.shields.io/crates/v/debil-dynamodb.svg)](https://crates.io/crates/debil-dynamodb)

DynamoDB implementation for [debil](https://github.com/myuon/debil).

This crate is "abusing" debil's macro. Some features make no sense in this crate. (e.g. `size` attribute for a column).

This library is inspired by [dynomite](https://github.com/softprops/dynomite).

## Features

- [x] Nullability checks in `into_item`
- [x] Safe accessor with `accessor!` macro (See [this](https://github.com/myuon/debil#accessor-macro))
- [ ] Safe deserialization (currently it panics if deserialization failed)

## How to use

Getting started with `Table` derive, which derives mapper into/from `HashMap<String, AttributeValue>` (record type in `rusoto_dynamodb`).

At this time, you must specify `table_name, sql_type, primary_key` in sql attribute.

```rust
use debil::*;
use debil_dynamodb::*;
use rusoto_dynamodb::AttributeValue;

#[derive(Table)]
#[sql(table_name = "foo", sql_type = "DynamoType", primary_key = "id")]
pub struct Foo {
    id: String,
    amount: i32,
    tags: Vec<String>,
}
```

Then you can convert it to/from attributes.

```rust
#[test]
fn foo_serialize() {
    let foo = Foo {
        id: "12345".to_string(),
        amount: 10,
        tags: vec!["abc".to_string(), "def".to_string()],
    };

    assert_eq!(
        into_item(foo),
        vec![
            (
                "id",
                AttributeValue {
                    s: Some("12345".to_string()),
                    ..Default::default()
                }
            ),
            (
                "amount",
                AttributeValue {
                    n: Some("10".to_string()),
                    ..Default::default()
                }
            ),
            (
                "tags",
                AttributeValue {
                    l: Some(vec![
                        AttributeValue {
                            s: Some("abc".to_string()),
                            ..Default::default()
                        },
                        AttributeValue {
                            s: Some("def".to_string()),
                            ..Default::default()
                        }
                    ]),
                    ..Default::default()
                }
            )
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
    );
}

#[test]
fn foo_serialize_empty_string_check() {
    let foo = Foo {
        id: "".to_string(),
        ..Default::default()
    };

    assert_eq!(
        into_item(foo)["id"],
        AttributeValue {
            null: Some(true),
            ..Default::default()
        }
    );
}
```

For call DynamoDB API, use [rusoto_dynamodb](https://crates.io/crates/rusoto_dynamodb).

