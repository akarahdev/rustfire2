use crate::api::flow::control::Control;
use crate::api::flow::handles::ElseHandle;
use crate::api::headers::functions::Functions;
use crate::api::items::any::Any;
use crate::api::items::dict::Dictionary;
use crate::api::items::string::String;
use crate::api::items::VarItem;
use crate::api::selections::targets::EventDefault;
use crate::core::args::TemplateItem;
use crate::items::{comp, num, str};
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct Optional<T: VarItem>(Dictionary<String, Any>, PhantomData<T>);

impl<T: VarItem> VarItem for Optional<T> {
    fn as_item(&self) -> TemplateItem {
        self.0.clone().as_item()
    }

    fn from_item(item: TemplateItem) -> Self {
        Optional(Dictionary(item, PhantomData), PhantomData)
    }

    fn default() -> Self {
        Optional::empty()
    }
}

impl<T: VarItem> Optional<T> {
    pub fn empty() -> Optional<T> {
        Functions::declare_with_return(Functions::allocate_name(), move || {
            let dict: Dictionary<String, Any> = Dictionary::new();
            dict.put(str!("is_present"), Any::from_value(num!(0)));
            Optional::from_item(dict.as_item())
        })
    }

    pub fn wrap(inner: T) -> Optional<T> {
        Functions::declare_with_return(Functions::allocate_name(), move || {
            let dict: Dictionary<String, Any> = Dictionary::new();
            dict.put(str!("is_present"), Any::from_value(num!(1)));
            dict.put(str!("value"), Any::from_value(inner));
            Optional::from_item(dict.as_item())
        })
    }

    pub fn if_present<F: FnOnce()>(&self, f: F) -> ElseHandle {
        self.0
            .get(str!("is_present"))
            .if_equals(Any::from_value(num!(1)), f);
        ElseHandle
    }

    pub fn if_none<F: FnOnce()>(&self, f: F) -> ElseHandle {
        self.0
            .get(str!("is_present"))
            .if_equals(Any::from_value(num!(0)), f);
        ElseHandle
    }

    pub fn unwrap(&self) -> T {
        let s = self.clone();
        Functions::declare_with_return(Functions::allocate_name(), move || {
            s.if_none(|| {
                EventDefault::player()
                    .send_message(comp!("<red>Error: Used unwrap on None value."));
                Control::end_thread();
            });
            s.0.get(str!("value")).cast()
        })
    }

    pub fn unwrap_or(&self, other: T) -> T {
        let s = self.clone();
        Functions::declare_with_return(Functions::allocate_name(), move || {
            let mut out: T = T::default();
            s.if_none(|| {
                out = other;
            })
            .or_else(|| {
                out = s.0.get(str!("value")).cast();
            });
            out
        })
    }
}
