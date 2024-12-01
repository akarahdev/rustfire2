use crate::api::{push_block, start_new_template, CURRENT_TEMPLATE, TEMPLATE_REPOSITORY};
use crate::codetemplate::template::{Template, TemplateBlock};

pub struct Processes;

impl Processes {
    pub fn declare<F: FnOnce() + Send + 'static>(name: &str, f: F) {
        let name = name.to_string();
        start_new_template(move || {
            push_block(TemplateBlock::process(name));
            f();
        });
    }

    pub fn call(name: &str) {
        push_block(TemplateBlock::start_process(name.to_string()));
    }
}