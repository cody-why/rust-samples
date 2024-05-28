/*
 * @Author: plucky
 * @Date: 2023-09-17 23:04:15
 * @LastEditTime: 2023-09-17 23:16:09
 */

use std::{thread, time};

use specs::prelude::*;

// A component contains data
// which is associated with an entity.
#[derive(Debug)]
struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Pos(f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

struct SysA;

impl<'a> System<'a> for SysA {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        // The `.join()` combines multiple component storages,
        // so we get access to all entities which have
        // both a position and a velocity.
        for (pos, vel) in (&mut pos, &vel).join() {
            println!("{} {}", pos.0, vel.0);
            pos.0 += vel.0;
        }
        println!("---");
    }
}

fn main() {
    // The `World` is our
    // container for components
    // and other resources.
    let mut world = World::new();
    world.register::<Pos>();
    world.register::<Vel>();

    // An entity may or may not contain some component.

    world.create_entity().with(Vel(2.0)).with(Pos(0.0)).build();
    world.create_entity().with(Vel(4.0)).with(Pos(1.6)).build();
    world.create_entity().with(Vel(1.5)).with(Pos(5.4)).build();

    // This entity does not have `Vel`, so it won't be dispatched.
    world.create_entity().with(Pos(2.0)).build();

    // This builds a dispatcher.
    // The third parameter of `with` specifies
    // logical dependencies on other systems.
    // Since we only have one, we don't depend on anything.
    // See the `full` example for dependencies.
    let mut dispatcher = DispatcherBuilder::new().with(SysA, "sys_a", &[]).build();
    // This will call the `setup` function of every system.
    // In this example this has no effect since we already registered our components.
    dispatcher.setup(&mut world);

    // This dispatches all the systems in parallel (but blocking).
    dispatcher.dispatch(& world);

    world.maintain();

    for _ in 0..60 {
        dispatcher.dispatch(&world);
        thread::sleep(time::Duration::from_millis(1000));
    }

    // 创建一个时钟,每秒更新系统

    



}