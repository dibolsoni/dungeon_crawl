use crate::prelude::*;
use serde::Deserialize;
use ron::de::from_reader;
use std::fs::File;
use std::collections::HashSet;
use legion::systems::CommandBuffer;

#[derive(Deserialize, Clone, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<i32>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/templates.ron").expect("No file found");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn get(&self, name: &str) -> Option<&Template> {
        self.entities.iter().find(|t| t.name == name)
    }

    pub fn spawn_entities_per_level(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|t| t.levels.contains(&(level as i32)))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });
        let mut commands = CommandBuffer::new(ecs);
        spawn_points
            .iter()
            .for_each(|pt| {
                if let Some(entity) = rng.random_slice_entry(&available_entities) {
                    self.spawn_entity(pt, entity, &mut commands);
                }
            });
        commands.flush(ecs);
    }
    fn spawn_entity(
        &self,
        pt: &Point,
        template: &Template,
        commands: &mut CommandBuffer,
    ) {
        let entity = commands.push((
            pt.clone(),
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone()),
        ));
        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy);
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(entity, ChasingPlayer{});
                commands.add_component(entity, Health {
                    current: template.hp.unwrap(),
                    max: template.hp.unwrap(),
                });
            }
        }
        if let Some(effects) = &template.provides {
            effects.iter().for_each(|(effect, amount)| {
                match effect.as_str() {
                    "Healing" => {
                        commands.add_component(entity, ProvidesHealing { amount: *amount });
                    }
                    "MagicMap" => {
                        commands.add_component(entity, ProvidesDungeonMap {});
                    }
                    _ => println!("Unknown effect: {}", effect),
                }
            });
        }
        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon{});
            }
        }
    }
}
