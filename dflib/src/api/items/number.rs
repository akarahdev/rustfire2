use std::ops::{Add, Div, Mul, Sub};
use std::sync::atomic::Ordering;
use crate::api::items::refs::Ref;
use crate::api::{allocate_variable, push_block, CURRENT_TEMPLATE, VAR_INDEX};
use crate::api::items::VarItem;
use crate::codetemplate::args::{ChestArgs, Item, NamedData, VarData};
use crate::codetemplate::template::TemplateBlock;

#[derive(Debug, Clone)]
pub struct Number(pub(crate) Item);

impl VarItem for Number {
    fn as_item(&self) -> Item {
        self.0.clone()
    }
}

impl Number {
    pub fn new(raw: &str) -> Number {
        Number(Item::Number { data: NamedData { name: raw.to_string() } })
    }
}

impl Add for Number {
    type Output = Ref<Number>;

    fn add(self, other: Number) -> Ref<Number> {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "+".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.0)
                .with_slot(2, other.0),
        ));
        Ref(Number(result))
    }
}

impl Sub for Number {
    type Output = Ref<Number>;

    fn sub(self, other: Number) -> Ref<Number> {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "-".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.0)
                .with_slot(2, other.0),
        ));
        Ref(Number(result))
    }
}

impl Mul for Number {
    type Output = Ref<Number>;

    fn mul(self, other: Number) -> Ref<Number> {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "*".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.0)
                .with_slot(2, other.0),
        ));
        Ref(Number(result))
    }
}

impl Div for Number {
    type Output = Ref<Number>;

    fn div(self, other: Number) -> Ref<Number> {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "/".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.0)
                .with_slot(2, other.0),
        ));
        Ref(Number(result))
    }
}