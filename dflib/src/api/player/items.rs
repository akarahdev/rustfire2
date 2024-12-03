use crate::api::cf::time::Duration;
use crate::api::items::item::Item;
use crate::api::items::number::Number;
use crate::api::items::VarItem;
use crate::api::player::{player_action, Player};
use crate::api::{allocate_variable, push_block};
use crate::codetemplate::args::ChestArgs;
use crate::codetemplate::args::Item as DFItem;
use crate::codetemplate::template::{BlockType, TemplateBlock};

pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Helmet,
    Chestplate,
    Leggings,
    Boots,
}


player_action! {
    fn give_item => "GiveItems";

    arg item: Item;
}


player_action! {
    fn set_item_in_slot => "SetItemInSlot";

    arg item: Item;
    arg slot: Number;
}

player_action! {
    fn set_armor => "SetArmor";

    arg helmet: Item;
    arg chestplate: Item;
    arg leggings: Item;
    arg boots: Item;
}

player_action! {
    fn remove_item => "RemoveItems";

    arg item: Item;
}

player_action! {
    fn replace_item => "ReplaceItems";

    arg original: Item;
    arg new: Item;
}

player_action! {
    fn clear_item => "ClearItems";

    arg item: Item;
}


player_action! {
    fn set_cursor_item => "SetCursorItem";

    arg item: Item;
}

player_action! {
    fn save_inventory => "SaveInv";
}

player_action! {
    fn load_inventory => "LoadInv";
}

player_action! {
    fn set_item_cooldown => "SetItemCooldown";

    arg item: Item;
    arg cooldown: Number;
}

player_action! {
    fn get_item_cooldown => "GetItemCooldown" => Number;

    arg item: Item;
}

impl Player {
    pub fn give_items(&self, items: &[Item]) {
        let mut args = ChestArgs::new();
        for item in items.iter().enumerate() {
            args = args.with_slot(item.0 as u8, item.1.as_item())
        }
        push_block(TemplateBlock::player_action(
            "GiveItems".to_string(),
            "Selection".to_string(),
            args,
        ))
    }

    pub fn set_equipment_slot(&self, slot: EquipmentSlot, item: Item) {
        push_block(TemplateBlock::player_action(
            "SetEquipment".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, item.as_item())
                .with_slot(26, DFItem::block_tag(
                    match slot {
                        EquipmentSlot::MainHand => "Main hand",
                        EquipmentSlot::OffHand => "Off hand",
                        EquipmentSlot::Helmet => "Helmet",
                        EquipmentSlot::Chestplate => "Chestplate",
                        EquipmentSlot::Leggings => "Leggings",
                        EquipmentSlot::Boots => "Boots",
                    },
                    "EquipmentSlot",
                    "SetEquipment",
                    BlockType::PlayerAction,
                )),
        ))
    }
}