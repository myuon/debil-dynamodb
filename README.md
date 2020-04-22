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
// call GetItem API
let body = client
    .query(rusoto_dynamodb::QueryInput {
        ...
    })
    .await?;

let items = body.items.unwrap();
debil_dynamodb::from_items(items) // <- Vec<Foo>

// call PutItem API
client
    .put_item(rusoto_dynamodb::PutItemInput {
        // Use into_item API to serialize
        item: into_item(Foo {
            id: "12345".to_string(),
            amount: 10,
            tags: vec!["1".to_string(), "2".to_string()]
        }),
        ..Default::default()
    })
    .await?;
```

For call DynamoDB API, use [rusoto_dynamodb](https://crates.io/crates/rusoto_dynamodb).

