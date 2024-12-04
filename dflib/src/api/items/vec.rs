use crate::api::{allocate_variable, push_block};
use crate::api::items::number::Number;
use crate::api::items::{set_variable, TypedVarItem, VarItem};
use crate::codetemplate::args::{ChestArgs, Item, VecData};
use crate::codetemplate::template::TemplateBlock;

#[derive(Debug, Clone, Copy)]
pub struct Vector(pub(crate) Item);
impl TypedVarItem for Vector {}

impl VarItem for Vector {
    fn as_item(&self) -> Item {
        self.0.clone()
    }

    fn from_item(item: Item) -> Self {
        Vector(item)
    }

    fn default() -> Self {
        Self::new_const(0.0, 0.0, 0.0)
    }
}

impl Vector {
    pub fn new_const(x: f64, y: f64, z: f64) -> Vector {
        Vector(Item::Vector {
            data: VecData {
                x,
                y,
                z,
            }
        })
    }

    pub fn new_dyn(x: Number, y: Number, z: Number) -> Vector {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "Vector".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, x.as_item().clone())
                .with_slot(2, y.as_item().clone())
                .with_slot(3, z.as_item().clone()),
        ));
        Vector(result)
    }
}

set_variable! {
    impl Vector; fn (x => "GetVectorComp") -> Number;
    tag "Component" => "X";
}

set_variable! {
    impl Vector; fn (y => "GetVectorComp") -> Number;
    tag "Component" => "Y";
}

set_variable! {
    impl Vector; fn (z => "GetVectorComp") -> Number;
    tag "Component" => "z";
}

set_variable! {
    impl Vector; fn (with_x => "SetVectorComp") -> Vector;
    arg value: Number;
    tag "Component" => "X";
}

set_variable! {
    impl Vector; fn (with_y => "SetVectorComp") -> Vector;
    arg value: Number;
    tag "Component" => "Y";
}

set_variable! {
    impl Vector; fn (with_z => "SetVectorComp") -> Vector;
    arg value: Number;
    tag "Component" => "Z";
}