use crate::api::{push_block, CURRENT_TEMPLATE, TEMPLATE_REPOSITORY};
use crate::codetemplate::template::{Template, TemplateBlock};

pub struct Processes;

impl Processes {
    pub fn declare<F: FnOnce()>(name: &str, f: F) {
        CURRENT_TEMPLATE.lock().unwrap().blocks = vec![];
        push_block(TemplateBlock::process(name.to_string()));
        f();
        TEMPLATE_REPOSITORY.lock().unwrap().push(Template { blocks: CURRENT_TEMPLATE.lock().unwrap().blocks.clone() });
    }

    pub fn call(name: &str) {
        push_block(TemplateBlock::start_process(name.to_string()));
    }
}