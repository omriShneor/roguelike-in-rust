mod components;
mod map;
mod visibility_system;
mod player;
pub mod rect;
use player::player_input;
use visibility_system::VisibilitySystem;
use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use components::{Position, Renderable, Player, Viewshed};
use map::{TileType, Map};

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    for y in 0..map.height {
        for x in 0..map.width {
            let idx = map.xy_idx(x,y);
            if map.revealed_tiles[idx] {
                match map.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x,y, RGB::from_f32(0.5,0.5,0.5), RGB::from_f32(0.,0.,0.), rltk::to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x,y, RGB::from_f32(0.0,1.0,0.0), RGB::from_f32(0.,0.,0.), rltk::to_cp437('#'));
                    }
                }
            }
        }
    }
}


pub struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.run_systems();
        player_input(self, ctx);
        draw_map(&self.ecs, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State {
        ecs: World::new()
    };

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    gs.ecs.insert(map);
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

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
