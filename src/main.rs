mod components;
mod map;
mod visibility_system;
mod player;
mod monster_ai_system;
pub mod rect;
use player::player_input;
use visibility_system::VisibilitySystem;
use monster_ai_system::MonsterAI;
use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use components::{Position, Renderable, Player, Viewshed, Monster, RunState};
use map::{TileType, Map};

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    for y in 0..map.height {
        for x in 0..map.width {
            let idx = map.xy_idx(x,y);
            if map.revealed_tiles[idx] {
                let glyph;
                let mut fg;
                match map.tiles[idx] {
                    TileType::Floor => {
                        glyph = rltk::to_cp437('.');
                        fg = RGB::from_f32(0.0, 0.5, 0.5);
                    }
                    TileType::Wall => {
                        glyph = rltk::to_cp437('#');
                        fg = RGB::from_f32(0., 1.0, 0.);
                    }
                }
                if !map.visiable_tiles[idx] {fg = fg.to_greyscale()}
                ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
            }
        }
    }
}


pub struct State {
    ecs: World,
    pub runstate: RunState
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }
        draw_map(&self.ecs, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visiable_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running
    };

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();


    let mut rng = rltk::RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let (x,y) = room.center();
        let glyph: rltk::FontCharType;
        let roll = rng.roll_dice(1,2);
        match roll {
            1 => { glyph = rltk::to_cp437('g')},
            _ => { glyph = rltk::to_cp437('o')}
        }
        gs.ecs.create_entity()
            .with(Position{ x, y })
            .with(Renderable{
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
            .with(Monster{})
            .build();
    }
    
    gs.ecs.insert(map);
    gs.ecs
        .create_entity()
        .with(Position {x: player_x, y: player_y})
        .with(Renderable {
        glyph: rltk::to_cp437('@'),
        fg: RGB::named(rltk::YELLOW),
        bg: RGB::named(rltk::BLACK), 
        })
        .with(Player{})
        .with(Viewshed{visible_tiles: Vec::new(), range:8, dirty: true})
        .build();

    rltk::main_loop(context, gs)
}
