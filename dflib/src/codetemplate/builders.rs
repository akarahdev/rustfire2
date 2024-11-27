use crate::codetemplate::args::ChestArgs;
use crate::codetemplate::template::{BlockId, BlockType, BracketDirection, BracketType, TemplateBlock};

impl TemplateBlock {
    pub const fn block_id(mut self, id: BlockId) -> Self {
        self.id = id;
        self
    }

    pub const fn block(mut self, ty: BlockType) -> Self {
        self.block = Some(ty);
        self
    }

    pub fn action(mut self, v: String) -> Self {
        self.action = Some(v);
        self
    }

    pub fn sub_action(mut self, v: String) -> Self {
        self.sub_action = Some(v);
        self
    }

    pub fn target(mut self, v: String) -> Self {
        self.target = Some(v);
        self
    }

    pub fn data(mut self, v: String) -> Self {
        self.data = Some(v);
        self
    }

    pub fn attribute(mut self, v: String) -> Self {
        self.attribute = Some(v);
        self
    }

    pub fn bracket_direction(mut self, dir: BracketDirection) -> Self {
        self.bracket_direction = Some(dir);
        self
    }

    pub fn bracket_type(mut self, ty: BracketType) -> Self {
        self.bracket_type = Some(ty);
        self
    }

    pub fn args(mut self, args: ChestArgs) -> Self {
        self.args = Some(args);
        self
    }
}

impl TemplateBlock {
    pub fn bracket(direction: BracketDirection, ty: BracketType) -> TemplateBlock {
        TemplateBlock::default()
            .block_id(BlockId::Bracket)
            .bracket_direction(direction)
            .bracket_type(ty)
    }

    pub fn else_block() -> TemplateBlock {
        TemplateBlock::default()
            .block(BlockType::Else)
    }

    pub fn player_event(event: String) -> TemplateBlock {
        TemplateBlock::default()
            .block(BlockType::PlayerEvent)
            .action(event)
    }

    pub fn player_action(
        action: String,
        target: String,
        args: ChestArgs
    ) -> TemplateBlock {
        TemplateBlock::default()
            .block(BlockType::PlayerAction)
            .action(action)
            .target(target)
            .args(args)
    }

    pub fn entity_action(
        action: String,
        target: String,
        args: ChestArgs
    ) -> TemplateBlock {
        TemplateBlock::default()
            .block(BlockType::EntityAction)
            .action(action)
            .target(target)
            .args(args)
    }

    pub fn game_action(
        action: String,
        args: ChestArgs
    ) -> TemplateBlock {
        TemplateBlock::default()
            .block(BlockType::GameAction)
            .action(action)
            .args(args)
    }

    pub fn set_variable(
        action: String,
        args: ChestArgs
    ) -> TemplateBlock {
        TemplateBlock::default()
            .block(BlockType::SetVariable)
            .action(action)
            .args(args)
    }

    pub fn select_object(
        action: String,
        args: ChestArgs
    ) -> TemplateBlock {
        TemplateBlock::default()
            .block(BlockType::SelectObject)
            .action(action)
            .args(args)
    }
}
