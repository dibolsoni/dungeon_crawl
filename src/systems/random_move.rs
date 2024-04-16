use rand::Rng;
use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut rand = rand::thread_rng();
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    movers
        .iter_mut(ecs)
        .for_each(|(entity, pos, _)| {
            let destination = match rand.gen_range(0..4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;
            commands.push((
                (),
                WantsToMove { entity: *entity, destination }
            ));
        });
}
