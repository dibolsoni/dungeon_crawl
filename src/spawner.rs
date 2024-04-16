use rand::Rng;
use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            },
        )
    );
}

pub fn spawn_enemy(ecs: &mut World, pos: Point) {
    let mut rand = rand::thread_rng();
    ecs.push(
        (
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: match rand.gen_range(0..4) {
                    0 => to_cp437('E'),
                    1 => to_cp437('O'),
                    2 => to_cp437('o'),
                    _ => to_cp437('g'),
                },
            },
            MovingRandomly{},
        )
    );
}
