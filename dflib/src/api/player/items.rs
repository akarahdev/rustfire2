use crate::api::cf::time::Duration;
use crate::api::items::item::Item;
use crate::api::items::number::Number;
use crate::api::items::VarItem;
use crate::api::player::Player;
use crate::api::{allocate_variable, push_block};
use crate::codetemplate::args::ChestArgs;
use crate::codetemplate::args::Item as DFItem;
use crate::codetemplate::template::{BlockType, TemplateBlock};
use crate::num;

pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Helmet,
    Chestplate,
    Leggings,
    Boots,
}

impl Player {
    pub fn give_item(&self, item: Item) {
        push_block(TemplateBlock::player_action(
            "GiveItems".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, item.as_item()),
        ))
    }

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

    pub fn set_item_in_slot(&self, slot: Number, item: Item) {
        push_block(TemplateBlock::player_action(
            "GiveItems".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, item.as_item())
                .with_slot(1, slot.as_item()),
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

    pub fn replace_item(&self, original: Item, new: Item) {
        push_block(TemplateBlock::player_action(
            "ReplaceItems".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, original.as_item())
                .with_slot(1, new.as_item()),
        ))
    }

    pub fn clear_item(&self, item: Item) {
        push_block(TemplateBlock::player_action(
            "ClearItems".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, item.as_item()),
        ))
    }

    pub fn clear_inv(&self) {
        push_block(TemplateBlock::player_action(
            "ClearItems".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(25, DFItem::block_tag("True", "Clear Crafting and Cursor",
                                                 "ClearItems", BlockType::PlayerAction))
                .with_slot(26, DFItem::block_tag("Entire inventory", "Clear mode",
                                                 "ClearItems", BlockType::PlayerAction)),
        ))
    }

    pub fn set_cursor_item(&self, item: Item) {
        push_block(TemplateBlock::player_action(
            "SetCursorItem".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, item.as_item())
        ))
    }

    pub fn save_inventory(&self) {
        push_block(TemplateBlock::player_action(
            "SaveInv".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
        ))
    }

    pub fn load_inventory(&self) {
        push_block(TemplateBlock::player_action(
            "LoadInv".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
        ))
    }

    pub fn set_item_cooldown(&self, item: Item, cooldown: Duration) {
        push_block(TemplateBlock::player_action(
            "SetItemCooldown".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, item.as_item())
                .with_slot(1, Number::new(&*cooldown.0.to_string()).as_item())
        ))
    }

    pub fn get_item_cooldown(&self, item: Item) -> Number {
        let result = allocate_variable();
        push_block(TemplateBlock::player_action(
            "GetItemCooldown".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, item.as_item())
        ));
        Number(result)
    }
}