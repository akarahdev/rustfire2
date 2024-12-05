use crate::api::items::any::Any;
use crate::api::items::dict::Dictionary;
use crate::api::player::Player;
use crate::core::args::VarData;
use std::marker::PhantomData;

impl Player {
    pub fn game_data(&self) -> Dictionary<crate::api::items::string::String, Any> {
        Dictionary(
            crate::core::args::TemplateItem::Variable {
                data: VarData {
                    name: "rf/%uuid/d",
                    scope: "unsaved",
                },
            },
            PhantomData,
        )
    }

    pub fn saved_data(&self) -> Dictionary<crate::api::items::string::String, Any> {
        Dictionary(
            crate::core::args::TemplateItem::Variable {
                data: VarData {
                    name: "rf/%uuid/sd",
                    scope: "saved",
                },
            },
            PhantomData,
        )
    }
}
