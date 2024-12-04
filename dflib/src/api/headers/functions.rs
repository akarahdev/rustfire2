use std::sync::Arc;
use std::sync::mpsc::channel;
use crate::api::{allocate_variable, push_block, start_new_template};
use crate::api::items::VarItem;
use crate::codetemplate::args::Item;
use crate::codetemplate::template::TemplateBlock;

pub struct Functions;

impl Functions {
    pub fn allocate_name() -> &'static str {
        let Item::Variable { data } = allocate_variable() else { unreachable!() ; };
        data.name
    }

    pub fn declare<F: FnOnce() + Send + 'static>(name: &str, f: F) {
        let str = name.to_string();
        start_new_template(|| {
            push_block(TemplateBlock::function(str));
            f();
        });
    }

    pub fn declare_with_return<O: VarItem + 'static, F: FnOnce() -> O + Send + 'static>(name: &'static str, f: F) -> O {
        Functions::call(name);

        let (sender, recv) = channel();
        start_new_template(move || {
            push_block(TemplateBlock::function(name.to_string()));
            let out = f();
            let returned = Arc::new(out);
            sender.send(returned).unwrap();
        });

        (*recv.recv().unwrap().clone()).clone()
    }

    pub fn call(name: &str) {
        push_block(TemplateBlock::call_function(name.to_string()));
    }
}