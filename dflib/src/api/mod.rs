pub mod items;
pub mod cf;
pub mod selections;
pub mod player;
pub mod entity;
pub mod headers;
mod config;

use std::cell::UnsafeCell;
use std::cmp::max;
use std::ops::Deref;
use std::path::Path;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex};
use std::time::Instant;
use crate::api::config::{Config, PlotRank};
use crate::codetemplate::args::{Item, VarData};
use crate::codetemplate::codeclient::send_to_code_client;
use crate::codetemplate::template::{Template, TemplateBlock};

thread_local! {
    // Safety: This is safe due to how this is accessed.
    // RustFire only accesses the current template thread-locally to write when it is compiling.
    // Only after it compiles the current template does it attempt to read from it.
    // There is no mutability accesses while trying to do read-only accesses.
    pub static CURRENT_TEMPLATE: UnsafeCell<Template> = UnsafeCell::new(Template { blocks: vec![] });
}

pub static TEMPLATE_REPOSITORY: Mutex<Vec<Template>> = Mutex::new(vec![]);
pub static VAR_INDEX: AtomicU64 = AtomicU64::new(0);
pub static THREAD_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub static COMPILER_CONFIG: LazyLock<Config> = LazyLock::new(|| read_config());

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

pub(crate) fn start_new_template<F: FnOnce() + Send + 'static>(f: F) {
    // CURRENT_TEMPLATE.lock().unwrap().blocks = vec![];
    THREAD_COUNTER.fetch_add(1, Ordering::AcqRel);
    std::thread::spawn(move || {
        CURRENT_TEMPLATE.with(|tmp| {
            unsafe { (*tmp.get()).blocks = vec![]; };
        });
        f();
        end_template();
    });
}

pub(crate) fn end_template() {
    CURRENT_TEMPLATE.with(|tmp| {
        let blocks = unsafe { (*tmp.get()).blocks.clone() };
        TEMPLATE_REPOSITORY.lock().unwrap().push(Template { blocks });
    });
    THREAD_COUNTER.fetch_sub(1, Ordering::AcqRel);
}

pub(crate) fn push_block(template_block: TemplateBlock) {
    CURRENT_TEMPLATE.with(|tmp| {
        unsafe { (*tmp.get()).blocks.push(template_block); }
    });
}

pub(crate) fn read_config() -> Config {
    let file = std::fs::read_to_string(Path::new("./Rustfire.toml"))
        .expect("expected a Rustfire.toml at crate root");
    let config: Config = toml::from_str(&file).expect("expected a validly formatted Rustfire.toml");
    config
}

pub(crate) fn assert_rank(rank: PlotRank) {
    let current_rank = &COMPILER_CONFIG.owner.rank;
    if (*current_rank as u8) < (rank as u8) {
        panic!("One of your actions requires {:?} to use!", rank);
    }
}

pub fn done() {
    let start = Instant::now();
    let mut time = 1;
    while THREAD_COUNTER.load(Ordering::Acquire) != 0 {
        std::thread::sleep(std::time::Duration::from_nanos(time));
        time += 1;
    }
    let end = Instant::now();
    println!("{:?}", end - start);
    
    let templates = TEMPLATE_REPOSITORY.lock().unwrap().clone();

    let max_length = COMPILER_CONFIG.plot.size as u16;
    for template in &templates {
        if template.blocks.len() > max_length as usize {
            panic!("Error on block {:?}: {} blocks exceeds max length of {}",
                template.blocks.first().unwrap(), template.blocks.len(), max_length);
        }
    }
    
    send_to_code_client(templates);
}