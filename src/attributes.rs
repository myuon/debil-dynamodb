use crate::Attribute;
use debil::{SQLTable, SQLValue};
use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct DynamoType(AttributeValue);

impl DynamoType {
    pub fn from_attr(attr: AttributeValue) -> Self {
        DynamoType(attr)
    }

    pub fn into_attr(self) -> AttributeValue {
        self.0
    }
}

// Thanks to this impl, you can do `#[sql(sql_type = "AttributeValue")]`
impl<T: Attribute> SQLValue<T> for DynamoType {
    fn column_type(_: PhantomData<T>, size: i32) -> String {
        unimplemented!()
    }

    fn serialize(item: T) -> Self {
        DynamoType::from_attr(item.into_attr())
    }

    fn deserialize(self) -> T {
        // oooooooops!
        Attribute::from_attr(self.into_attr()).unwrap()
    }
}

pub fn into_item_unchecked<T: SQLTable<ValueType = DynamoType>>(
    item: T,
) -> HashMap<String, AttributeValue> {
    item.map_to_sql()
        .into_iter()
        .map(|(k, v)| (k, v.into_attr()))
        .collect()
}

/// This will transform an empty string into null value, since DynamoDB does not allow an empty string
pub fn into_item<T: SQLTable<ValueType = DynamoType>>(item: T) -> HashMap<String, AttributeValue> {
    into_item_unchecked(item)
        .into_iter()
        .map(|(k, v)| {
            let value = if let Some(rep) = v.s {
                (if rep.len() == 0 { None } else { Some(rep) }).into_attr()
            } else {
                v
            };

            (k, value)
        })
        .collect()
}
