use crate::api::{push_block, start_new_template};
use crate::codetemplate::template::TemplateBlock;

pub struct Functions;

impl Functions {
    pub fn declare<F: FnOnce() + Send + 'static>(name: &str, f: F) {
        let str = name.to_string();
        start_new_template(|| {
            push_block(TemplateBlock::function(str));
            f();
        });
    }

    pub fn call(name: &str) {
        push_block(TemplateBlock::call_function(name.to_string()));
    }
}