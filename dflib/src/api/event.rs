use crate::api::player::Player;
use crate::api::{push_block, CURRENT_TEMPLATE, TEMPLATE_REPOSITORY};
use crate::api::selections::targets::EventDefault;
use crate::codetemplate::args::{ChestArgs, Item};
use crate::codetemplate::template::{BlockType, Template, TemplateBlock};

pub struct Functions;

impl Functions {
    pub fn declare<F: FnOnce()>(name: &str, f: F) {
        CURRENT_TEMPLATE.lock().unwrap().blocks = vec![];
        push_block(TemplateBlock::function(name.to_string()));
        f();
        TEMPLATE_REPOSITORY.lock().unwrap().push(Template { blocks: CURRENT_TEMPLATE.lock().unwrap().blocks.clone() });
    }

    pub fn call(name: &str) {
        push_block(TemplateBlock::call_function(name.to_string()));
    }
}

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

pub struct PlayerEvent;

#[macro_export]
macro_rules! subscribe {
    (
        plot_id: $name:expr;
        $($remainder:tt)*
    ) => {
        fn main() {
            subscribe!($($remainder)*);
        }
    };
    ($function_name:ident for $category:ident::$event_name:ident();
    $($remainder:tt)*) => {
        $(rustfire::api::event::$category::$event_name($function_name);)*
        subscri
    };
    () => { rustfire::api::done(); }
}

#[macro_use]
macro_rules! default_event {
    ($($(#[doc = $comment:expr])? $name:ident as $sign:expr),*) => {
        $(
            $(
                #[doc = $comment]
            )?
            pub fn $name<F: FnOnce(EventDefault<Player>)>(f: F) {
                CURRENT_TEMPLATE.lock().unwrap().blocks = vec![];
                push_block(TemplateBlock::player_event($sign.to_string()));
                f(EventDefault(Player));
                TEMPLATE_REPOSITORY.lock().unwrap().push(Template { blocks: CURRENT_TEMPLATE.lock().unwrap().blocks.clone() });
            }
        )*
    };
}

impl PlayerEvent {
    default_event! {
        #[doc = "Executes code when a player joins the plot."]
        join as "Join",
        #[doc = "Executes code when a player leaves the plot."]
        leave as "Leave",
        #[doc = "Executes code when a player right clicks."]
        right_click as "RightClick",
        #[doc = "Executes code when a player left clicks."]
        left_click as "LeftClick",
        #[doc = "Executes code when a player right clicks an entity."]
        right_click_entity as "RightClickEntity",
        #[doc = "Executes code when a player left clicks an entity."]
        left_click_entity as "LeftClickEntity",
        #[doc = "Executes code when a player places a block."]
        place_block as "PlaceBlock",
        #[doc = "Executes code when a player breaks a block."]
        break_block as "BreakBlock",
        #[doc = "Executes code when a player clicks a slot in a custom menu."]
        click_menu_slot as "ClickMenuSlot",
        #[doc = "Executes code when a player clicks a slot in their inventory."]
        click_inv_slot as "ClickInvSlot",
        #[doc = "Executes code when a player clicks a slot in a container."]
        click_container_slot as "ClickContainerSlot",
        #[doc = "Executes code when a player respawns."]
        respawn as "Respawn"
    }
}