use bytes::Bytes;
use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub enum AttributeError {
    MissingField(String),
    InvalidFormat(Box<dyn Error>),
}

pub trait Attribute: Sized {
    fn into_attr(self) -> AttributeValue;
    fn from_attr(attr: AttributeValue) -> Result<Self, AttributeError>;
}

impl Attribute for Bytes {
    fn into_attr(self) -> AttributeValue {
        AttributeValue {
            b: Some(self),
            ..Default::default()
        }
    }

    fn from_attr(attr: AttributeValue) -> Result<Self, AttributeError> {
        attr.b.ok_or(AttributeError::MissingField("B".to_string()))
    }
}

impl Attribute for bool {
    fn into_attr(self) -> AttributeValue {
        AttributeValue {
            bool: Some(self),
            ..Default::default()
        }
    }

    fn from_attr(attr: AttributeValue) -> Result<Self, AttributeError> {
        attr.bool
            .ok_or(AttributeError::MissingField("BOOL".to_string()))
    }
}

pub struct BSVec(pub Vec<Bytes>);

impl Attribute for BSVec {
    fn into_attr(self) -> AttributeValue {
        AttributeValue {
            bs: Some(self.0),
            ..Default::default()
        }
    }

    fn from_attr(attr: AttributeValue) -> Result<Self, AttributeError> {
        let items = attr
            .bs
            .ok_or(AttributeError::MissingField("BS".to_string()))?;
        Ok(BSVec(items))
    }
}

impl<T: Attribute> Attribute for Vec<T> {
    fn into_attr(self) -> AttributeValue {
        AttributeValue {
            l: Some(self.into_iter().map(|a| a.into_attr()).collect::<Vec<_>>()),
            ..Default::default()
        }
    }

    fn from_attr(attr: AttributeValue) -> Result<Self, AttributeError> {
        let vec = attr
            .l
            .ok_or(AttributeError::MissingField("L".to_string()))?;

        vec.into_iter()
            .map(|v| Attribute::from_attr(v))
            .collect::<Result<_, _>>()
    }
}

impl<T: Attribute> Attribute for HashMap<String, T> {
    fn into_attr(self) -> AttributeValue {
        AttributeValue {
            m: Some(self.into_iter().map(|(k, v)| (k, v.into_attr())).collect()),
            ..Default::default()
        }
    }

    fn from_attr(attr: AttributeValue) -> Result<Self, AttributeError> {
        let hm = attr
            .m
            .ok_or(AttributeError::MissingField("M".to_string()))?;

        hm.into_iter()
            .map(|(k, v)| {
                let w = Attribute::from_attr(v)?;

                Ok((k, w))
            })
            .collect::<Result<_, _>>()
    }
}

macro_rules! derive_number_for {
    ($t: tt) => {
        impl Attribute for $t {
            fn into_attr(self) -> AttributeValue {
                AttributeValue {
                    n: Some(self.to_string()),
                    ..Default::default()
                }
            }

            fn from_attr(attr: AttributeValue) -> Result<Self, AttributeError> {
                let rep = attr
                    .n
                    .ok_or(AttributeError::MissingField("N".to_string()))?;
                rep.parse()
                    .map_err(|err| AttributeError::InvalidFormat(Box::new(err)))
            }
        }
    };
}

derive_number_for!(i32);
derive_number_for!(u32);
derive_number_for!(i64);
derive_number_for!(u64);
derive_number_for!(f32);
derive_number_for!(f64);

pub struct NSVec<T>(pub Vec<T>);

impl<T: Attribute + ToString> Attribute for NSVec<T> {
    fn into_attr(self) -> AttributeValue {
        AttributeValue {
            ns: Some(self.0.into_iter().map(|t| t.to_string()).collect()),
            ..Default::default()
        }
    }

    fn from_attr(attr: AttributeValue) -> Result<Self, AttributeError> {
        let vec = attr
            .ns
            .ok_or(AttributeError::MissingField("NS".to_string()))?;

        vec.into_iter()
            .map(|rep| {
                Attribute::from_attr(AttributeValue {
                    n: Some(rep),
                    ..Default::default()
                })
            })
            .collect::<Result<_, _>>()
            .map(|v| NSVec(v))
    }
}

impl<T: Attribute> Attribute for Option<T> {
    fn into_attr(self) -> AttributeValue {
        match self {
            Some(t) => t.into_attr(),
            None => AttributeValue {
                null: Some(true),
                ..Default::default()
            },
        }
    }

    fn from_attr(attr: AttributeValue) -> Result<Self, AttributeError> {
        if attr.null.is_some() {
            return Ok(None);
        }

        Attribute::from_attr(attr).map(|v| Some(v))
    }
}

impl Attribute for String {
    fn into_attr(self) -> AttributeValue {
        AttributeValue {
            s: Some(self),
            ..Default::default()
        }
    }

    fn from_attr(attr: AttributeValue) -> Result<Self, AttributeError> {
        attr.s.ok_or(AttributeError::MissingField("S".to_string()))
    }
}

pub struct SSVec(pub Vec<String>);

impl Attribute for SSVec {
    fn into_attr(self) -> AttributeValue {
        AttributeValue {
            ss: Some(self.0),
            ..Default::default()
        }
    }

    fn from_attr(attr: AttributeValue) -> Result<Self, AttributeError> {
        attr.ss
            .ok_or(AttributeError::MissingField("SS".to_string()))
            .map(|v| SSVec(v))
    }
}
