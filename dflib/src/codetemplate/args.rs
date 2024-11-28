use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;
use valence_nbt::{Compound, Value};
use valence_nbt::snbt::{SnbtReader, SnbtWriter};
use crate::codetemplate::template::BlockType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChestArgs {
    items: Vec<Argument>
}

impl ChestArgs {
    pub fn new() -> ChestArgs {
        ChestArgs { items: vec![] }
    }

    pub fn with_slot(mut self, slot: u8, item: Item) -> Self {
        self.items.push(Argument { slot, item });
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Argument {
    item: Item,
    slot: u8
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "id")]
pub enum Item {
    #[serde(rename = "txt")]
    String { data: NamedData },
    #[serde(rename = "comp")]
    Component { data: NamedData },
    #[serde(rename = "num")]
    Number { data: NamedData },
    #[serde(rename = "var")]
    Variable { data: VarData },
    #[serde(rename = "bl_tag")]
    BlockTag { data: BlockTagData },
    #[serde(rename = "loc")]
    Location { data: LocData },
    #[serde(rename = "vec")]
    Vector { data: VecData },
    #[serde(rename = "item")]
    MCItem { data: VanillaItemData },
}

impl Item {
    pub fn string(name: &str) -> Self {
        Item::String { data: NamedData { name: name.to_string() }}
    }

    pub fn number(name: &str) -> Self {
        Item::Number { data: NamedData { name: name.to_string() }}
    }

    pub fn component(name: &str) -> Self {
        Item::Component { data: NamedData { name: name.to_string() }}
    }

    pub fn block_tag(
        option: &str,
        tag: &str,
        action: &str,
        block: BlockType
    ) -> Item {
        Item::BlockTag { data: BlockTagData {
            option: option.to_string(),
            tag: tag.to_string(),
            action: action.to_string(),
            block
        }}
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NamedData {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockTagData {
    pub option: String,
    pub tag: String,
    pub action: String,
    pub block: BlockType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VarData {
    pub name: String,
    pub scope: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocData {
    #[serde(rename = "isBlock")]
    pub is_block: bool,
    pub loc: LocValue
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocValue {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f64,
    pub yaw: f64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VecData {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

/// FIXME: make deserialization work, eventually...
#[derive(Debug, Clone, Deserialize)]
pub struct VanillaItemData {
    pub item: Compound
}

impl Serialize for VanillaItemData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut buf = String::new();
        let mut writer = SnbtWriter::new(&mut buf);
        writer.write_element(&Value::Compound(self.item.clone()));
        let mut strct = serializer.serialize_struct("VanillaItemData", 1)?;
        strct.serialize_field("item", &writer.to_string())?;
        strct.end()
    }
}