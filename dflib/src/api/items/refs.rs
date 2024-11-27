use std::ops::Deref;
use crate::api::items::VarItem;
use crate::codetemplate::args::Item;

pub struct Ref<T: VarItem>(pub(crate) T);

impl<T: VarItem> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: VarItem> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Ref(self.0.clone())
    }
}

impl<T: VarItem> VarItem for Ref<T> {
    fn as_item(&self) -> Item {
        self.0.as_item()
    }
}