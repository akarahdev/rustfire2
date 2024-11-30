pub mod items;
pub mod cf;
pub mod selections;
pub mod player;
pub mod entity;
pub mod headers;

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use crate::codetemplate::args::{Item, VarData};
use crate::codetemplate::codeclient::send_to_code_client;
use crate::codetemplate::template::{Template, TemplateBlock};

pub static CURRENT_TEMPLATE: Mutex<Template> = Mutex::new(Template { blocks: vec![] });
pub static TEMPLATE_REPOSITORY: Mutex<Vec<Template>> = Mutex::new(vec![]);
pub static VAR_INDEX: AtomicU64 = AtomicU64::new(0);

/// Use this macro to register your DiamondFire events for plots.
#[macro_export]
macro_rules! registry {
    ($b:block) => {
        fn main() {
            $b;
            $crate::api::done();
        }
    };
}

/// Use this macro to register DiamondFire events for libraries.
#[macro_export]
macro_rules! export_registry {
    ($b:block) => {
        fn registry() {
            $b;
        }
    };
}

pub(crate) fn allocate_variable() -> Item {
    Item::Variable {
        data: VarData {
            name: VAR_INDEX.fetch_add(1, Ordering::AcqRel).to_string(),
            scope: "line".to_string(),
        }
    }
}

pub(crate) fn push_block(template_block: TemplateBlock) {
    CURRENT_TEMPLATE.lock().unwrap().blocks.push(template_block);
}

pub fn done() {
    let templates = TEMPLATE_REPOSITORY.lock().unwrap().clone();
    send_to_code_client(templates);
}