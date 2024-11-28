use std::marker::PhantomData;
use std::ops::Index;
use crate::api::{allocate_variable, push_block};
use crate::api::items::list::List;
use crate::api::items::VarItem;
use crate::codetemplate::args::{ChestArgs, Item};
use crate::codetemplate::template::TemplateBlock;

pub struct Dictionary<K: VarItem, V: VarItem>(Item, PhantomData<(K, V)>);

impl<K: VarItem, V: VarItem> Clone for Dictionary<K, V> {
    fn clone(&self) -> Self {
        Dictionary(self.0.clone(), PhantomData)
    }
}

impl<K: VarItem, V: VarItem> VarItem for Dictionary<K, V> {
    fn as_item(&self) -> Item {
        self.0.clone()
    }

    fn from_item(item: Item) -> Self {
        Dictionary(item, PhantomData)
    }
}

impl<K: VarItem, V: VarItem> Dictionary<K, V> {
    pub fn new() -> Dictionary<K, V> {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "CreateDict".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
        ));
        Dictionary(result, PhantomData)
    }

    pub fn from_lists(keys: List<K>, values: List<V>) -> Dictionary<K, V> {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "CreateDict".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, keys.as_item())
                .with_slot(2, values.as_item())
        ));
        Dictionary(result, PhantomData)
    }

    pub fn put(&self, key: K, value: V) {
        push_block(TemplateBlock::set_variable(
            "SetDictValue".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item().clone())
                .with_slot(1, key.as_item().clone())
                .with_slot(2, value.as_item().clone())
        ))
    }

    pub fn get(&self, key: K) -> V {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "GetDictValue".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.as_item().clone())
                .with_slot(2, key.as_item().clone())
        ));
        V::from_item(result)
    }
}