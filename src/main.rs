mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;
mod turn_state;


mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::components::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        map_builder.monster_spawns
            .iter()
            .for_each(|(pos)|
                spawn_enemy(&mut ecs, &mut rng, *pos)
            );
        spawn_player(&mut ecs, map_builder.player_start);
        spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }


    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3);
        ctx.print_centered(5, "Your quest has ended.");
        ctx.print_centered(7, "Slain by a monster, your hero's journey has come to a close.");
        ctx.print_centered(9, "Press 1 to play again.");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3);
        ctx.print_centered(5, "You have reached the amulet of YALA!");
        ctx.print_centered(7, "You grab it and run back to the surface.");
        ctx.print_centered(9, "Victory is yours!");
        ctx.print_centered(11, "Press 1 to play again.");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|room| spawner::spawn_enemy(&mut self.ecs, &mut rng, room));
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        ctx.set_active_console(3);
        ctx.cls();
        self.resources.insert(ctx.key);
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self.input_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self.player_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self.monster_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
        }
        render_draw_buffer(ctx).expect("Render error");
    }

}

fn main() {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_sparse_console_no_bg(DISPLAY_WIDTH*4, DISPLAY_HEIGHT*4, "terminal8x8.png")
        .build()
        .unwrap();

    main_loop(context, State::new()).expect("TODO: panic message");
}
