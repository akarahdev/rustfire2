use serde::{Deserialize, Serialize};
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