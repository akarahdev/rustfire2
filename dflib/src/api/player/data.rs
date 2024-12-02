use std::marker::PhantomData;
use crate::api::items::any::Any;
use crate::api::items::dict::Dictionary;
use crate::api::player::Player;
use crate::codetemplate::args::VarData;

impl Player {
    pub fn game_data(&self) -> Dictionary<crate::api::items::string::String, Any> {
        Dictionary(
            crate::codetemplate::args::Item::Variable { data: VarData { name: "rf/%uuid/d".to_string(), scope: "unsaved".to_string() } },
            PhantomData
        )
    }

    pub fn saved_data(&self) -> Dictionary<crate::api::items::string::String, Any> {
        Dictionary(
            crate::codetemplate::args::Item::Variable { data: VarData { name: "rf/%uuid/sd".to_string(), scope: "saved".to_string() } },
            PhantomData
        )
    }
}