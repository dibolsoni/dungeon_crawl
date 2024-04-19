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
            Health { current: 20, max: 20 },
        )
    );
}

pub fn spawn_enemy(ecs: &mut World, pos: Point) {
    let mut rand = rand::thread_rng();
    let (hp, name, glyph) = match rand.gen_range(1..=8) {
        1..=5 => goblin(),
        _ => orc(),
    };
    ecs.push(
        (
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph,
            },
            ChasingPlayer{},
            Health { current: hp, max: hp },
            Name(name)
        )
    );
}


fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}


pub fn spawn_amulet_of_yala(
    ecs: &mut World,
    pos: Point
) {
    ecs.push(
        (
            Item,
            AmuletOfYala,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('|'),
            },
            Name("Amulet of Yala".to_string())
        )
    );
}
