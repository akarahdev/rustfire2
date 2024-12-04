pub mod comms;
pub mod cond;
pub mod data;
pub mod inventory;
pub mod items;
pub mod movement;
pub mod world;
mod visuals;

use crate::api::selections::Selection;

#[derive(Debug, Copy, Clone)]
pub struct Player;

impl Selection for Player {
    type Base = Player;

    fn selection_mechanism(&self) {}
}

pub(crate) macro player_action {
    (
        $(requires $rank:expr;)?
        fn $name:ident => $action:expr;
        $(arg $arg_name:ident : $arg_type:ty;)*
        $(tag $tag_name:expr => $tag_value:expr;)*
    ) => {
        impl $crate::api::player::Player {
            pub fn $name(&self, $($arg_name: $arg_type),*) {
                $($crate::api::assert_rank($rank);)?
                let v: Vec<&'static str> = vec![$($tag_value,)*];
                let _length = v.len() as u8;
                $crate::api::push_block($crate::codetemplate::template::TemplateBlock::player_action(
                    $action.to_string(),
                    "Selection".to_string(),
                    $crate::codetemplate::args::ChestArgs::new()
                        $(.with_slot($ {index()}, $arg_name.as_item()))*
                        $(
                            .with_slot(26-(_length - $ {index()} - 1),
                                $crate::codetemplate::args::Item::block_tag($tag_value, $tag_name,
                                $action, $crate::codetemplate::template::BlockType::PlayerAction)))*
                ));
            }
        }
    },
    (
        $(requires $rank:expr;)?
        fn $name:ident => $action:expr => $out:ty;
            $(arg $arg_name:ident : $arg_type:ty;)*
            $(tag $tag_name:expr => $tag_value:expr;)*
    ) => {
        impl $crate::api::player::Player {
            pub fn $name(&self, $($arg_name: $arg_type),*) -> $out {
                $($crate::api::assert_rank($rank);)?
                let result = $crate::api::allocate_variable();
                let v: Vec<&'static str> = vec![$($tag_value,)*];
                let _length = v.len() as u8;
                $crate::api::push_block($crate::codetemplate::template::TemplateBlock::player_action(
                    $action.to_string(),
                    "Selection".to_string(),
                    $crate::codetemplate::args::ChestArgs::new()
                        .with_slot(0, result.clone())
                        $(.with_slot(1+$ {index()}, $arg_name.as_item()))*
                        $(
                            .with_slot(26-(_length - $ {index()} - 1),
                                $crate::codetemplate::args::Item::block_tag($tag_value, $tag_name,
                                $action, $crate::codetemplate::template::BlockType::PlayerAction)))*
                ));
                <$out>::from_item(result.clone())
            }
        }
    }
}
