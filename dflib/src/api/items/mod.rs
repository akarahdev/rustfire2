pub mod component;
pub mod loc;
pub mod number;
pub mod string;
pub mod intos;
pub mod list;
pub mod dict;
pub mod vec;
pub mod any;
pub mod item;

use crate::codetemplate::args::Item;

pub trait VarItem: Clone {
    fn as_item(&self) -> Item;
    fn from_item(item: Item) -> Self;
}

pub trait TypedVarItem: VarItem {}

#[macro_export]
macro_rules! num {
    ($($t:tt)*) => { $crate::api::items::number::Number::new(stringify!($($t)*)) }
}

#[macro_export]
macro_rules! str {
    ($t:expr) => { $crate::api::items::string::String::new($t) }
}

#[macro_export]
macro_rules! comp {
    ($t:expr) => { $crate::api::items::component::Component::new($t) }
}

#[macro_export]
macro_rules! list {
    ($($t:expr),*) => {
        $crate::api::items::list::List::new_with_all(vec![$($t),*])
    }
}

#[macro_export]
macro_rules! dict {
    ($($key:expr => $value:expr),*) => {{
        let _lk = $crate::api::items::list::List::new_with_all(vec![$($key),*]);
        let _lv = $crate::api::items::list::List::new_with_all(vec![$($value),*]);
        $crate::api::items::dict::Dictionary::from_lists(_lk, _lv)
    }};
}

#[macro_export]
macro_rules! start {
    ($name:ident) => { $crate::api::headers::processes::Processes::call(stringify!($name)); }
}

#[macro_export]
macro_rules! call {
    ($name:ident) => { $crate::api::headers::functions::Functions::call(stringify!($name)); }
}

pub(crate) macro set_variable_with_self(

impl $inn:ty; fn ($name:ident => $action:expr) -> $out:ty;
        $(arg $arg_name:ident : $arg_type:ty;)*
        $(tag $tag_name:expr => $tag_value:expr;)*
) {
    impl $inn {
        pub fn $name(&self, $($arg_name: $arg_type),*) -> $out {
            let result = $crate::api::allocate_variable();
            $crate::api::push_block($crate::codetemplate::template::TemplateBlock::set_variable(
                $action.to_string(),
                $crate::codetemplate::args::ChestArgs::new()
                    .with_slot(0, result.clone())
                    .with_slot(1, self.as_item())
                    $(.with_slot(${index()}+2, $arg_name.as_item()))*
                    $(
                        .with_slot(26- ( ${len(0)} - ${index(0)} - 1),
                            $crate::codetemplate::args::Item::block_tag($tag_value, $tag_name,
                            $action, $crate::codetemplate::template::BlockType::PlayerAction)))*
            ));
            <$out>::from_item(result)
        }
    }
}