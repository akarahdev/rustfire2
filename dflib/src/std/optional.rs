use std::marker::PhantomData;
use crate::api::items::any::Any;
use crate::api::items::dict::Dictionary;
use crate::api::items::string::String;
use crate::api::items::VarItem;

// fieldS:
// is_present = "true" | "false"
// value = instance of T, if "is_present" = "true"
pub struct Optional<T: VarItem>(Dictionary<String, Any>, PhantomData<T>);
