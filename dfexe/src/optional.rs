use std::marker::PhantomData;
use rustfire::api::items::dict::Dictionary;
use rustfire::api::items::string::String;
use rustfire::api::items::VarItem;
use rustfire::api::items::any::Any;
use rustfire::{comp, num, str};
use rustfire::api::cf::control::Control;
use rustfire::api::headers::functions::Functions;
use rustfire::api::selections::targets::EventDefault;
use rustfire::codetemplate::args::Item as DFItem;

#[derive(Clone, Copy)]
pub struct Optional<T: VarItem> {
    dict: Dictionary<String, Any>,
    phantom: PhantomData<T>,
}

impl<T: VarItem> VarItem for Optional<T> {
    fn as_item(&self) -> DFItem {
        self.dict.as_item()
    }

    fn from_item(item: DFItem) -> Self {
        Optional { dict: Dictionary::from_item(item), phantom: PhantomData }
    }

    fn default() -> Self {
        Optional::empty()
    }
}

impl<T: VarItem> Optional<T> {
    pub fn new(inner: T) -> Optional<T> {
        Functions::declare_with_return(Functions::allocate_name(), move || {
            let dict: Dictionary<String, Any> = Dictionary::new();
            dict.put(str!("value"), Any::from_value(inner));
            dict.put(str!("is_present"), num!(1).into());
            Optional { dict, phantom: PhantomData }
        })
    }

    pub fn empty() -> Optional<T> {
        Functions::declare_with_return(Functions::allocate_name(), move || {
            let dict: Dictionary<String, Any> = Dictionary::new();
            dict.put(str!("is_present"), num!(0).into());
            Optional { dict, phantom: PhantomData }
        })
    }

    pub fn unwrap(&self) -> T {
        let s = self.clone();
        Functions::declare_with_return(Functions::allocate_name(), move || {
            let mut out = T::default();
            s.dict.get(str!("is_present")).if_equals(num!(1).into(), || {
                out = s.dict.get(str!("value")).cast();
            }).or_else(|| {
                EventDefault::player().send_message(comp!("<red>ERROR: Used unwrap on empty Optional<T>"));
                Control::end_thread();
            });
            out
        })
    }
}