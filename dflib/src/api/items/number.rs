use std::ops::{Add, Div, Mul, Sub};
use crate::api::{allocate_variable, push_block};
use crate::api::items::{TypedVarItem, VarItem};
use crate::codetemplate::args::{ChestArgs, Item, NamedData};
use crate::codetemplate::template::{BlockType, BracketDirection, BracketType, TemplateBlock};

#[derive(Debug, Clone, Copy)]
pub struct Number(pub(crate) Item);
impl TypedVarItem for Number {}

impl VarItem for Number {
    fn as_item(&self) -> Item {
        self.0.clone()
    }

    fn from_item(item: Item) -> Self {
        Number(item)
    }

    fn default() -> Self {
        Number::new("0")
    }
}

impl Number {
    pub fn new(raw: &'static str) -> Number {
        Number(Item::Number { data: NamedData { name: raw } })
    }

    pub fn random_int(min: Number, max: Number) -> Number {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "RandomNumber".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, min.as_item())
                .with_slot(2, max.as_item())
                .with_slot(26, Item::block_tag("Whole number", "Rounding Mode",
                        "RandomNumber", BlockType::SetVariable))
        ));
        Number(result)
    }

    pub fn random_decimal(min: Number, max: Number) -> Number {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "RandomNumber".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, min.as_item())
                .with_slot(2, max.as_item())
                .with_slot(26, Item::block_tag("Decimal number", "Rounding Mode",
                                               "RandomNumber", BlockType::SetVariable))
        ));
        Number(result)
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "+".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.0)
                .with_slot(2, other.0),
        ));
        Number(result)
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "-".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.0)
                .with_slot(2, other.0),
        ));
        Number(result)
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "*".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.0)
                .with_slot(2, other.0),
        ));
        Number(result)
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "/".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.0)
                .with_slot(2, other.0),
        ));
        Number(result)
    }
}

impl Number {
    pub fn if_greater_than<F: FnOnce()>(&self, other: Number, run: F) {
        push_block(TemplateBlock::if_variable(
            ">".to_string(),
            ChestArgs::new()
                .with_slot(0, self.clone().as_item())
                .with_slot(1, other.clone().as_item())
        ));
        push_block(TemplateBlock::bracket(BracketDirection::Start, BracketType::Normal));
        run();
        push_block(TemplateBlock::bracket(BracketDirection::End, BracketType::Normal));
    }

    pub fn if_less_than<F: FnOnce()>(&self, other: Number, run: F) {
        push_block(TemplateBlock::if_variable(
            "<".to_string(),
            ChestArgs::new()
                .with_slot(0, self.clone().as_item())
                .with_slot(1, other.clone().as_item())
        ));
        push_block(TemplateBlock::bracket(BracketDirection::Start, BracketType::Normal));
        run();
        push_block(TemplateBlock::bracket(BracketDirection::End, BracketType::Normal));
    }

    pub fn if_greater_than_or_equal<F: FnOnce()>(&self, other: Number, run: F) {
        push_block(TemplateBlock::if_variable(
            ">=".to_string(),
            ChestArgs::new()
                .with_slot(0, self.clone().as_item())
                .with_slot(1, other.clone().as_item())
        ));
        push_block(TemplateBlock::bracket(BracketDirection::Start, BracketType::Normal));
        run();
        push_block(TemplateBlock::bracket(BracketDirection::End, BracketType::Normal));
    }

    pub fn if_less_than_or_equal<F: FnOnce()>(&self, other: Number, run: F) {
        push_block(TemplateBlock::if_variable(
            "<=".to_string(),
            ChestArgs::new()
                .with_slot(0, self.clone().as_item())
                .with_slot(1, other.clone().as_item())
        ));
        push_block(TemplateBlock::bracket(BracketDirection::Start, BracketType::Normal));
        run();
        push_block(TemplateBlock::bracket(BracketDirection::End, BracketType::Normal));
    }
}