use debil::*;
use debil_dynamodb::*;
use rusoto_dynamodb::AttributeValue;

#[derive(Table, PartialEq, Default)]
#[sql(table_name = "foo", sql_type = "DynamoType", primary_key = "id")]
pub struct Foo {
    id: String,
    amount: i32,
    tags: Vec<String>,
}

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
