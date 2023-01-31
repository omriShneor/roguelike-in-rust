mod components;
mod map;
pub mod rect;
use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use components::{Position, Renderable, Player};
use map::{TileType, Map};


fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();
    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = map.xy_idx(pos.x+delta_x, pos.y+delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {} // Nothing happened.
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1,0,&mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1,0,&mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0,-1,&mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0,1,&mut gs.ecs),
            _ => {}
        },
    }
}

pub fn draw_map(map: &Map, ctx: &mut Rltk) {
    for y in 0..map.height {
        for x in 0..map.width {
            match map.tiles[map.xy_idx(x,y)] {
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


struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.run_systems();
        player_input(self, ctx);
        let map = self.ecs.fetch::<Map>();
        draw_map(&map, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
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

    gs.ecs
        .create_entity()
        .with(Position {x: player_x, y: player_y})
        .with(Renderable {
        glyph: rltk::to_cp437('@'),
        fg: RGB::named(rltk::YELLOW),
        bg: RGB::named(rltk::BLACK), 
        })
        .with(Player{})
        .build();

    rltk::main_loop(context, gs)
}
