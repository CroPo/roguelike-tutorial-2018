use ecs::id::EntityId;
use ecs::Ecs;
use message::Message;
use ecs::component::Actor;
use ecs::component::Name;

#[derive(Clone)]
pub enum Spell {
    Heal
}

impl Spell {
    pub fn execute(&self, ecs: &mut Ecs, caster_id: EntityId) -> Option<Message> {
        match *self {
            Spell::Heal => self.heal(ecs, caster_id)
        }
    }

    fn heal(&self, ecs: &mut Ecs, caster_id: EntityId) -> Option<Message> {

        let entity_name = Self::get_entity_name(ecs, caster_id);

        if let Some(actor) = ecs.get_component_mut::<Actor>(caster_id) {
            actor.hp = actor.max_hp;
            Some(Message::new(format!("{} was fully healed", entity_name)))
        }
        else {
            None
        }
    }

    fn get_entity_name(ecs: &Ecs, id: EntityId) -> String {
        match ecs.get_component::<Name>(id) {
            Some(n) => n.name.clone(),
            None => format!("a nameless entity (#{})", id)
        }
    }
}
