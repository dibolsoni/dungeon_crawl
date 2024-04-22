use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
#[read_component(Name)]
pub fn use_item(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    let mut activated_item = <(Entity, &ActivateItem)>::query();
    activated_item.iter(ecs).for_each(|(entity, activate)| {
        let item = ecs.entry_ref(activate.item).unwrap();
        if let Ok(healing) = item.get_component::<ProvidesHealing>() {
            healing_to_apply.push((activate.used_by, healing.amount));
        }
        if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
            map.revealed_tiles.iter_mut().for_each(|t| *t = true);
        }
        commands.remove(activate.item);
        commands.remove(*entity);
    });

    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1);
            }
        }
    }
}
