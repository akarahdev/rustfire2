use crate::api::items::number::Number;
use crate::api::items::{set_variable, TypedVarItem, VarItem};
use crate::api::{allocate_variable, push_block};
use crate::core::args::{ChestArgs, TemplateItem, LocData, LocValue};
use crate::core::template::{BlockType, TemplateBlock};

#[derive(Debug, Clone, Copy)]
pub struct Location(pub(crate) TemplateItem);
impl TypedVarItem for Location {}

impl VarItem for Location {
    fn as_item(&self) -> TemplateItem {
        self.0.clone()
    }

    fn from_item(item: TemplateItem) -> Self {
        Location(item)
    }

    fn default() -> Self {
        Location::new_const(0.0, 0.0, 0.0)
    }
}

set_variable! {
    impl Location; fn (with_x => "SetCoord") -> Location;
    arg coord: Number;
    tag "Coordinate Type" => "Plot Coordinate";
    tag "Coordinate" => "X";
}

set_variable! {
    impl Location; fn (x => "GetCoord") -> Number;
    tag "Coordinate Type" => "Plot Coordinate";
    tag "Coordinate" => "X";
}

set_variable! {
    impl Location; fn (with_y => "SetCoord") -> Location;
    arg coord: Number;
    tag "Coordinate Type" => "Plot Coordinate";
    tag "Coordinate" => "Y";
}

set_variable! {
    impl Location; fn (y => "GetCoord") -> Number;
    tag "Coordinate Type" => "Plot Coordinate";
    tag "Coordinate" => "Y";
}

set_variable! {
    impl Location; fn (with_z => "SetCoord") -> Location;
    arg coord: Number;
    tag "Coordinate Type" => "Plot Coordinate";
    tag "Coordinate" => "Z";
}

set_variable! {
    impl Location; fn (z => "GetCoord") -> Number;
    tag "Coordinate Type" => "Plot Coordinate";
    tag "Coordinate" => "Z";
}

impl Location {
    pub fn new_const(x: f64, y: f64, z: f64) -> Location {
        Location(TemplateItem::Location {
            data: LocData {
                is_block: false,
                loc: LocValue {
                    x,
                    y,
                    z,
                    pitch: 0.0,
                    yaw: 0.0,
                },
            },
        })
    }

    pub fn new_const_angled(x: f64, y: f64, z: f64, pitch: f64, yaw: f64) -> Location {
        Location(TemplateItem::Location {
            data: LocData {
                is_block: false,
                loc: LocValue {
                    x,
                    y,
                    z,
                    pitch,
                    yaw,
                },
            },
        })
    }

    pub fn new_dyn(x: Number, y: Number, z: Number) -> Location {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "SetAllCoords".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, x.as_item().clone())
                .with_slot(2, y.as_item().clone())
                .with_slot(3, z.as_item().clone())
                .with_slot(
                    26,
                    TemplateItem::block_tag(
                        "Plot coordinate",
                        "Coordinate Type",
                        "SetAllCoords",
                        BlockType::SetVariable,
                    ),
                ),
        ));
        Location(result)
    }

    pub fn new_dyn_angled(x: Number, y: Number, z: Number, pitch: Number, yaw: Number) -> Location {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "SetAllCoords".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, x.as_item().clone())
                .with_slot(2, y.as_item().clone())
                .with_slot(3, z.as_item().clone())
                .with_slot(4, pitch.as_item().clone())
                .with_slot(5, yaw.as_item().clone())
                .with_slot(
                    26,
                    TemplateItem::block_tag(
                        "Plot coordinate",
                        "Coordinate Type",
                        "SetAllCoords",
                        BlockType::SetVariable,
                    ),
                ),
        ));
        Location(result)
    }
}
