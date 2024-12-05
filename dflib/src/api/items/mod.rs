pub mod any;
pub mod component;
pub mod dict;
pub mod item;
pub mod list;
pub mod loc;
pub mod number;
pub mod string;
pub mod vec;
pub mod cell;
pub mod particle;

pub use any::*;
pub use component::*;
pub use dict::*;
pub use item::*;
pub use list::*;
pub use loc::*;
pub use number::*;
pub use string::*;
pub use vec::*;
pub use cell::*;
pub use particle::*;

use crate::api::flow::handles::ElseHandle;
use crate::api::push_block;
use crate::core::args::{ChestArgs, TemplateItem};
use crate::core::template::{BracketDirection, BracketType, TemplateBlock};

pub trait VarItem: Clone + Copy + Send + Sync + 'static {
    fn as_item(&self) -> TemplateItem;
    fn from_item(item: TemplateItem) -> Self;
    fn default() -> Self;

    fn if_equals<F: FnOnce()>(&self, other: Self, run: F) -> ElseHandle {
        push_block(TemplateBlock::if_variable(
            "=".to_string(),
            ChestArgs::new()
                .with_slot(0, self.clone().as_item())
                .with_slot(1, other.clone().as_item()),
        ));
        push_block(TemplateBlock::bracket(
            BracketDirection::Start,
            BracketType::Normal,
        ));
        run();
        push_block(TemplateBlock::bracket(
            BracketDirection::End,
            BracketType::Normal,
        ));
        ElseHandle
    }
}

pub trait TypedVarItem: VarItem {}

pub macro num($($t:tt)*) {
    $crate::api::items::number::Number::new(stringify!($($t)*))
}

pub macro str($t:expr) {
    $crate::api::items::string::String::new($t)
}

pub macro comp($t:expr) {
    $crate::api::items::component::Component::new($t)
}

pub macro list($($t:expr),*) {
    $crate::api::items::list::List::new_with_all(vec![$($t),*])
}

pub macro dict($($key:expr => $value:expr),*) {{
    let _lk = $crate::api::items::list::List::new_with_all(vec![$($key),*]);
    let _lv = $crate::api::items::list::List::new_with_all(vec![$($value),*]);
    $crate::api::items::dict::Dictionary::from_lists(_lk, _lv)
}}

pub(crate) macro set_variable(
impl $inn:ty; fn ($name:ident => $action:expr) -> $out:ty;
        $(arg $arg_name:ident : $arg_type:ty;)*
        $(tag $tag_name:expr => $tag_value:expr;)*
) {
    impl $inn {
        pub fn $name(&self, $($arg_name: $arg_type),*) -> $out {
            let _v: Vec<&'static str> = vec![$($tag_value,)*];
            let _length = _v.len() as u8;
            let result = $crate::api::allocate_variable();
            $crate::push_block($crate::core::template::TemplateBlock::set_variable(
                $action.to_string(),
                $crate::core::args::ChestArgs::new()
                    .with_slot(0, result.clone())
                    .with_slot(1, self.as_item())
                    $(.with_slot(${index()}+2, $arg_name.as_item()))*
                    $(
                        .with_slot(26- (_length - ${index()} - 1),
                            $crate::core::args::TemplateItem::block_tag($tag_value, $tag_name,
                            $action, $crate::core::template::BlockType::SetVariable)))*
            ));
            <$out>::from_item(result)
        }
    }
}
