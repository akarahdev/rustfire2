pub mod data;
pub mod items;
pub mod comms;
pub mod movement;
pub mod world;
pub mod cond;

use crate::api::items::component::Component;
use crate::api::selections::Selection;
use crate::api::items::VarItem;

#[derive(Debug, Copy, Clone)]
pub struct Player;

impl Selection for Player {
    type Base = Player;

    fn selection_mechanism(&self) {

    }
}

pub(crate) macro player_action(
fn $name:ident => $action:expr;
        $(arg $arg_name:ident : $arg_type:ty;)*
        $(tag $tag_name:expr => $tag_value:expr;)*
) {
    impl $crate::api::player::Player {
        pub fn $name(&self, $($arg_name: $arg_type),*) {
            let v: Vec<&'static str> = vec![$($tag_value,)*];
            let length = v.len() as u8;
            $crate::api::push_block($crate::codetemplate::template::TemplateBlock::player_action(
                $action.to_string(),
                "Selection".to_string(),
                $crate::codetemplate::args::ChestArgs::new()
                    $(.with_slot(${index()}, $arg_name.as_item()))*
                    $(
                        .with_slot(26-(length - ${index()} - 1),
                            $crate::codetemplate::args::Item::block_tag($tag_value, $tag_name,
                            $action, $crate::codetemplate::template::BlockType::PlayerAction)))*
            ));
        }
    }
}