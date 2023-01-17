use crate::{Ecs, EntityDefinition};

pub struct CommandQueue {
    commands: Vec<Box<dyn Command>>,
}

impl CommandQueue {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }

    pub fn insert<ED>(&mut self, entity_definition: ED)
    where
        ED: 'static + EntityDefinition,
    {
        self.commands
            .push(Box::new(InsertEntityCommand::new(entity_definition)))
    }

    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Box<dyn Command>>,
    {
        self.commands.extend(iter);
    }

    pub fn drain(&mut self) -> std::vec::Drain<Box<dyn Command>> {
        self.commands.drain(..)
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Command {
    fn execute(self: Box<Self>, ecs: &mut Ecs);
}

pub struct InsertEntityCommand<ED>
where
    ED: EntityDefinition,
{
    entity_definition: ED,
}

impl<ED> InsertEntityCommand<ED>
where
    ED: EntityDefinition,
{
    pub fn new(entity_definition: ED) -> Self {
        Self { entity_definition }
    }
}

impl<ED> Command for InsertEntityCommand<ED>
where
    ED: EntityDefinition,
{
    fn execute(self: Box<Self>, ecs: &mut Ecs) {
        ecs.insert(self.entity_definition);
    }
}
