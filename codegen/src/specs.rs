use internal::*;

use amq_protocol_types::*;
use serde_json::{self, Value};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AMQProtocolDefinition {
    pub name:          ShortString,
    pub major_version: ShortShortUInt,
    pub minor_version: ShortShortUInt,
    pub revision:      ShortShortUInt,
    pub port:          LongUInt,
    pub copyright:     LongString,
    pub domains:       Vec<AMQPDomain>,
    pub constants:     Vec<AMQPConstant>,
    pub classes:       Vec<AMQPClass>,
}

impl AMQProtocolDefinition {
    pub fn load() -> AMQProtocolDefinition {
        let specs = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/specs/amqp-rabbitmq-0.9.1.json"));

        serde_json::from_str::<_AMQProtocolDefinition>(specs).expect("Failed to parse AMQP specs file").to_specs()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AMQPDomain {
    pub name:      ShortString,
    #[serde(rename="type")]
    pub amqp_type: AMQPType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AMQPConstant {
    pub name:  ShortString,
    pub value: u16,
    #[serde(rename="class")]
    pub klass: Option<ShortString>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AMQPClass {
    pub id:         ShortUInt,
    pub methods:    Vec<AMQPMethod>,
    pub name:       ShortString,
    pub properties: Option<Vec<AMQPProperty>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AMQPMethod {
    pub id:          ShortUInt,
    pub arguments:   Vec<AMQPArgument>,
    pub name:        ShortString,
    pub synchronous: Option<Boolean>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AMQPArgument {
    #[serde(rename="type")]
    pub amqp_type:     Option<AMQPType>,
    pub name:          ShortString,
    pub default_value: Option<Value>, /* TODO: convert that to an AMQPValue */
    pub domain:        Option<ShortString>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AMQPProperty {
    #[serde(rename="type")]
    pub amqp_type: AMQPType,
    pub name:      ShortString,
}
