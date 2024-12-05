use crate::api::flow::control::Control;
use crate::api::flow::repeat::Repeat;
use crate::api::headers::functions::Functions;
use crate::api::items::any::Any;
use crate::api::items::cell::Cell;
use crate::api::items::dict::Dictionary;
use crate::api::items::list::List;
use crate::api::items::number::Number;
use crate::api::items::string::String;
use crate::api::items::VarItem;
use crate::core::args::TemplateItem;
use crate::items::{num, str};
use crate::std::optional::Optional;
use std::marker::PhantomData;

pub trait Iterator: Copy + Clone + Send + Sync + 'static {
    type Item: VarItem;

    fn next(&self) -> Optional<Self::Item>;

    fn for_each<F: Fn(Self::Item) + Send + Sync + 'static>(&self, f: F) {
        let s = self.clone();
        Functions::declare(Functions::allocate_name(), move || {
            Repeat::forever(|| {
                let item = s.next();
                item.if_present(|| {
                    f(item.unwrap());
                })
                .or_else(|| {
                    Control::stop_repeat();
                });
            })
        });
    }

    fn count<F: Fn(Self::Item) + Send + Sync + 'static>(&self) -> Number {
        let s = self.clone();
        Functions::declare_with_return(Functions::allocate_name(), move || {
            let mut c = num!(0);
            Repeat::forever(|| {
                let item = s.next();
                item.if_present(|| {
                    c = c + num!(1);
                })
                .or_else(|| {
                    Control::stop_repeat();
                });
            });
            c
        })
    }

    fn map<O, F>(self, f: F) -> MapIter<Self, O, F>
    where
        O: VarItem,
        F: Fn(Self::Item) -> O + Send + Sync + 'static,
        Self: Sized,
    {
        MapIter(self, f, PhantomData)
    }
}

#[derive(Clone, Copy)]
pub struct ListIter<T: VarItem>(Dictionary<String, Any>, PhantomData<T>);

impl<T: VarItem> VarItem for ListIter<T> {
    fn as_item(&self) -> TemplateItem {
        self.0.as_item()
    }

    fn from_item(item: TemplateItem) -> Self {
        ListIter(Dictionary::from_item(item), PhantomData)
    }

    fn default() -> Self {
        List::new().iter()
    }
}
impl<T: VarItem> Iterator for ListIter<T> {
    type Item = T;

    fn next(&self) -> Optional<Self::Item> {
        let s = self.clone();
        Functions::declare_with_return(Functions::allocate_name(), move || {
            let cell = Cell::wrap(Optional::empty());
            s.0.get(str!("idx")).cast::<Number>().if_less_than_or_equal(
                s.0.get(str!("list")).cast::<List<T>>().len(),
                || {
                    let idx = s.0.get(str!("idx")).cast::<Number>();
                    s.0.put(str!("idx"), Any::from_value(idx + num!(1)));
                    cell.set(Optional::wrap(
                        s.0.get(str!("list")).cast::<List<T>>().get(idx),
                    ));
                },
            );
            cell.into_inner()
        })
    }
}

impl<T: VarItem> List<T> {
    pub fn iter(&self) -> ListIter<T> {
        let s = self.clone();
        Functions::declare_with_return(Functions::allocate_name(), move || {
            let dict = Dictionary::new();
            dict.put(str!("idx"), Any::from_value(num!(1)));
            dict.put(str!("list"), Any::from_value(s));
            ListIter(dict, PhantomData)
        })
    }
}

#[derive(Copy, Clone)]
pub struct MapIter<I: Iterator, O: VarItem, F: Fn(I::Item) -> O + Send + Sync + 'static>(
    I,
    F,
    PhantomData<O>,
);

impl<I: Iterator, O: VarItem, F: Fn(I::Item) -> O + Send + Sync + Copy + 'static> Iterator
    for MapIter<I, O, F>
{
    type Item = O;

    fn next(&self) -> Optional<Self::Item> {
        let s = self.clone();
        Functions::declare_with_return(Functions::allocate_name(), move || {
            let o = Cell::empty();
            let child = s.0.next();
            child
                .if_present(|| {
                    o.set(Optional::wrap(s.1(child.unwrap())));
                })
                .or_else(|| {
                    o.set(Optional::empty());
                });
            o.into_inner()
        })
    }
}
