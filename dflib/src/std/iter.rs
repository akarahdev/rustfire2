use crate::api::items::VarItem;
use crate::std::optional::Optional;

pub trait Iterator {
    type Item: VarItem;

    fn next(&self) -> Optional<Self::Item>;
}