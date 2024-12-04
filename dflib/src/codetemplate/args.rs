use crate::codetemplate::template::BlockType;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ChestArgs {
    items: Vec<Argument>,
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

#[derive(Serialize, Debug, Clone)]
pub struct Argument {
    item: Item,
    slot: u8,
}

#[derive(Serialize, Debug, Clone, Copy)]
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
    #[serde(rename = "pot")]
    Potion { data: PotionData },
    #[serde(rename = "g_val")]
    GameValue { data: GameValueData },
    #[serde(rename = "sound")]
    Sound { data: SoundData },
    #[serde(rename = "part")]
    Particle { data: ParticleData },
}

impl Item {
    pub fn string(name: &'static str) -> Self {
        Item::String {
            data: NamedData { name },
        }
    }

    pub fn number(name: &'static str) -> Self {
        Item::Number {
            data: NamedData { name },
        }
    }

    pub fn component(name: &'static str) -> Self {
        Item::Component {
            data: NamedData { name },
        }
    }

    pub fn block_tag(
        option: &'static str,
        tag: &'static str,
        action: &'static str,
        block: BlockType,
    ) -> Item {
        Item::BlockTag {
            data: BlockTagData {
                option,
                tag,
                action,
                block,
            },
        }
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct NamedData {
    pub name: &'static str,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct BlockTagData {
    pub option: &'static str,
    pub tag: &'static str,
    pub action: &'static str,
    pub block: BlockType,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct VarData {
    pub name: &'static str,
    pub scope: &'static str,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct LocData {
    #[serde(rename = "isBlock")]
    pub is_block: bool,
    pub loc: LocValue,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct LocValue {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f64,
    pub yaw: f64,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct VecData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct PotionData {
    pub pot: &'static str,
    pub dur: i64,
    pub amp: i64
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct SoundData {
    pub sound: &'static str,
    pub pitch: f64,
    pub vol: f64
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct GameValueData {
    #[serde(rename = "type")]
    pub value: &'static str,
    pub target: &'static str,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct ParticleData {
    pub particle: &'static str,
    pub cluster: ParticleCluster
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct ParticleCluster {
    pub amount: i64,
    pub horizontal: f64,
    pub vertical: f64
}