use crate::api::{allocate_variable, push_block};
use crate::api::items::loc::Location;
use crate::api::items::number::Number;
use crate::api::items::{TypedVarItem, VarItem};
use crate::codetemplate::args::{BlockTagData, ChestArgs, Item, LocData, LocValue, NamedData, VarData, VecData};
use crate::codetemplate::template::{BlockType, TemplateBlock};

#[derive(Debug, Clone)]
pub struct Vector(pub(crate) Item);
impl TypedVarItem for Vector {}

impl VarItem for Vector {
    fn as_item(&self) -> Item {
        self.0.clone()
    }

    fn from_item(item: Item) -> Self {
        Vector(item)
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

    pub fn from_between_locations(loc1: Location, loc2: Location) -> Vector {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "VectorBetween".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, loc1.as_item().clone())
                .with_slot(2, loc2.as_item().clone()),
        ));
        Vector(result)
    }

    pub fn x(&self) -> Number {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "GetVectorComp".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.as_item().clone())
                .with_slot(26, Item::block_tag("X", "Component",
                                               "GetVectorComp", BlockType::SetVariable)),
        ));
        Number(result)
    }

    pub fn y(&self) -> Number {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "GetVectorComp".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.as_item().clone())
                .with_slot(26, Item::block_tag("Y", "Component",
                                               "GetVectorComp", BlockType::SetVariable)),
        ));
        Number(result)
    }

    pub fn z(&self) -> Number {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "GetVectorComp".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.as_item().clone())
                .with_slot(26, Item::block_tag("Z", "Component",
                                               "GetVectorComp", BlockType::SetVariable)),
        ));
        Number(result)
    }

    pub fn with_x(&self, new_coord: Number) -> Vector {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "SetVectorComp".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.as_item().clone())
                .with_slot(2, new_coord.as_item().clone())
                .with_slot(26, Item::block_tag("X", "Component",
                                               "SetVectorComp", BlockType::SetVariable)),
        ));
        Vector(result)
    }

    pub fn with_y(&self, new_coord: Number) -> Vector {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "SetVectorComp".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.as_item().clone())
                .with_slot(2, new_coord.as_item().clone())
                .with_slot(26, Item::block_tag("Y", "Component",
                                               "SetVectorComp", BlockType::SetVariable)),
        ));
        Vector(result)
    }

    pub fn with_z(&self, new_coord: Number) -> Vector {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "SetVectorComp".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.as_item().clone())
                .with_slot(2, new_coord.as_item().clone())
                .with_slot(26, Item::block_tag("Z", "Component",
                                               "SetVectorComp", BlockType::SetVariable)),
        ));
        Vector(result)
    }
}