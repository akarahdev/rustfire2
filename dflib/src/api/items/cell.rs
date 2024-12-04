use std::marker::PhantomData;
use crate::api::{allocate_variable, push_block};
use crate::api::items::VarItem;
use crate::codetemplate::args::{ChestArgs, Item as DFItem};
use crate::codetemplate::template::{Template, TemplateBlock};

#[derive(Copy, Clone)]
pub struct Cell<T: VarItem>(DFItem, PhantomData<T>);

impl<T: VarItem> VarItem for Cell<T> {
    fn as_item(&self) -> DFItem {
        self.0.clone()
    }

    fn from_item(item: DFItem) -> Self {
        Cell(item, PhantomData::default())
    }

    fn default() -> Self {
        Cell(allocate_variable(), PhantomData::default())
    }
}

impl<T: VarItem> Cell<T> {
    pub fn empty() -> Cell<T> {
        Self::default()
    }
    
    pub fn wrap(value: T) -> Cell<T> {
        let cell = Cell::empty();
        cell.set(value);
        cell
    }
    
    pub fn set(&self, value: T) {
        push_block(TemplateBlock::set_variable(
            "=".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
                .with_slot(1, value.as_item())
        ))
    }
    
    pub fn into_inner(self) -> T {
        T::from_item(self.0)
    }
}