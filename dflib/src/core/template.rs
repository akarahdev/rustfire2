use crate::core::args::ChestArgs;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Template {
    pub blocks: Vec<TemplateBlock>,
}

#[derive(Serialize, Debug, Clone)]
pub enum BlockId {
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "bracket")]
    Bracket,
}

impl Default for BlockId {
    fn default() -> Self {
        BlockId::Block
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum BracketDirection {
    #[serde(rename = "open")]
    Start,
    #[serde(rename = "close")]
    End,
}

#[derive(Serialize, Debug, Clone)]
pub enum BracketType {
    #[serde(rename = "norm")]
    Normal,
    #[serde(rename = "repeat")]
    Sticky,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum BlockType {
    #[serde(rename = "call_func")]
    CallFunction,
    #[serde(rename = "control")]
    Control,
    #[serde(rename = "else")]
    Else,
    #[serde(rename = "entity_action")]
    EntityAction,
    #[serde(rename = "entity_event")]
    EntityEvent,
    #[serde(rename = "event")]
    PlayerEvent,
    #[serde(rename = "func")]
    Function,
    #[serde(rename = "game_action")]
    GameAction,
    #[serde(rename = "if_entity")]
    IfEntity,
    #[serde(rename = "if_game")]
    IfGame,
    #[serde(rename = "if_player")]
    IfPlayer,
    #[serde(rename = "if_var")]
    IfVariable,
    #[serde(rename = "player_action")]
    PlayerAction,
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "repeat")]
    Repeat,
    #[serde(rename = "select_obj")]
    SelectObject,
    #[serde(rename = "set_var")]
    SetVariable,
    #[serde(rename = "start_process")]
    StartProcess,
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct TemplateBlock {
    pub(crate) id: BlockId,

    #[serde(rename = "direct", skip_serializing_if = "Option::is_none")]
    pub(crate) bracket_direction: Option<BracketDirection>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub(crate) bracket_type: Option<BracketType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) block: Option<BlockType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) target: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) action: Option<String>,

    #[serde(rename = "subAction", skip_serializing_if = "Option::is_none")]
    pub(crate) sub_action: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) attribute: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) args: Option<ChestArgs>,
}
