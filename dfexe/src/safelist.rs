use rustfire::api::items::list::List;
use rustfire::api::items::number::Number;
use rustfire::api::items::VarItem;
use rustfire::codetemplate::args::Item;
use crate::optional::Optional;

#[derive(Clone, Copy)]
pub struct SafeList<T: VarItem> {
    inner: List<T>,
}

impl<T: VarItem> VarItem for SafeList<T> {
    fn as_item(&self) -> Item {
        self.inner.as_item()
    }

    fn from_item(item: Item) -> Self {
        SafeList { inner: List::from_item(item) }
    }

    fn default() -> Self {
        SafeList::new()
    }
}

impl<T: VarItem> SafeList<T> {
    pub fn new() -> Self {
        SafeList {
            inner: List::new()
        }
    }
    
    pub fn get(&self, index: Number) -> Optional<T> {
        let mut out = Optional::empty();
        index.if_less_than_or_equal(self.inner.len(), || {
            out = Optional::new(self.inner.get(index)); 
        });
        out
    }
    
    pub fn append(&self, value: T) -> &Self {
        self.inner.append(value);
        self
    }
}