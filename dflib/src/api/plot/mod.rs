use crate::items::VarItem;
use crate::items::{Item, Location};
pub struct Plot;

pub(crate) macro game_action(
fn $name:ident => $action:expr;
        $(arg $arg_name:ident : $arg_type:ty;)*
        $(tag $tag_name:expr => $tag_value:expr;)*
) {
    impl $crate::plot::Plot {
        pub fn $name($($arg_name: $arg_type),*) {
            let _v: Vec<&'static str> = vec![$($tag_value,)*];
            let _length = _v.len() as u8;
            $crate::push_block($crate::core::template::TemplateBlock::game_action(
                $action.to_string(),
                $crate::core::args::ChestArgs::new()
                    $(.with_slot(${index()}, $arg_name.as_item()))*
                    $(
                        .with_slot(26- (_length - ${index()} - 1),
                            $crate::core::args::TemplateItem::block_tag($tag_value, $tag_name,
                            $action, $crate::core::template::BlockType::SetVariable)))*
            ));
        }
    }
}

game_action! {
    fn set_block => "SetBlock";

    arg block: Item;
    arg loc: Location;
}
