
use specs::prelude::*;
use super::{Viewshed, Position, Monster};
use rltk::{console};

pub struct MonsterAI {}

 impl<'a> System<'a> for MonsterAI {
    type SystemData = (ReadStorage<'a, Viewshed>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, Monster>,
                    );

                        
    fn run(&mut self, data: Self::SystemData) {
        let (viewshed, pos, monster) = data;
        for (viewshed, pos, _monster) in (&viewshed, &pos, &monster).join() {
            console::log("Monster considers its own existence");
        }
    }
 }