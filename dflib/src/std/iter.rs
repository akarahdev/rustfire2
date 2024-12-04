use std::marker::PhantomData;
use crate::api::headers::functions::Functions;
use crate::api::items::any::Any;
use crate::api::items::dict::Dictionary;
use crate::api::items::list::List;
use crate::api::items::number::Number;
use crate::api::items::string::String;
use crate::api::items::VarItem;
use crate::codetemplate::args::Item;
use crate::std::optional::Optional;
use crate::{num, str};
use crate::api::items::cell::Cell;

pub trait Iterator {
    type Item: VarItem;

    fn next(&self) -> Optional<Self::Item>;
}

#[derive(Copy, Clone)]
pub struct ListIter<T: VarItem>(Dictionary<String, Any>, PhantomData<T>);

impl<T: VarItem> VarItem for ListIter<T> {
    fn as_item(&self) -> Item {
        self.0.as_item()
    }

    fn from_item(item: Item) -> Self {
        ListIter(Dictionary::from_item(item), PhantomData)
    }

    fn default() -> Self {
        ListIter(Dictionary::default(), PhantomData)
    }
}

impl<T: VarItem> Iterator for ListIter<T> {
    type Item = T;

    fn next(&self) -> Optional<Self::Item> {
        let s = self.clone();
        Functions::declare_with_return(Functions::allocate_name(), move || {
            let cell = Cell::wrap(Optional::empty());
            s.0.get(str!("idx")).cast::<Number>()
                .if_less_than_or_equal(s.0.get(str!("list")).cast::<List<T>>().len(), || {
                    let idx = s.0.get(str!("idx")).cast::<Number>();
                    s.0.put(str!("idx"), Any::from_value(idx + num!(1)));
                    cell.set(Optional::wrap(s.0.get(str!("list")).cast::<List<T>>().get(idx)));
            });
            cell.into_inner()
        })
    }
}

impl<T: VarItem> List<T> {
    pub fn iter(&self) -> ListIter<T> {
        let dict = Dictionary::new();
        dict.put(str!("idx"), Any::from_value(num!(1)));
        dict.put(str!("list"), Any::from_value(self.clone()));
        ListIter(dict, PhantomData)
    }
}