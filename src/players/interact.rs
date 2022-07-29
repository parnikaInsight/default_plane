use crate::players::info;
use crate::ggrs_rollback::network;
use bevy::prelude::*;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use std::env;

pub fn print_events(mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Hover! {:?}", e),
            PickingEvent::Clicked(e) => info!("Click! {:?}", e),
        }
    }
}

pub fn add_friend(
    mut events: EventReader<PickingEvent>,
    players: Query<(Entity, &Transform, &mut info::Player)>,
    mut me: Query<(Entity, &network::Me)>,
) {
    //i need to do this better
    // read cmd line arguments: 0 will be 7000, 1 will be 7001
    let args: Vec<String> = env::args().collect();
    let my_handle = &args[1];

    //println!("in add_friend");
    for event in events.iter() {
        match event {
            PickingEvent::Clicked(e) => {
                let mut added: bool = false;
                let mut id: u32 = 90;
                //spawn sprite bundle with transparent sprite background overlaid with text specific to player
                
                for i in players.iter() {
                    //if entity is the clicked one
                    if i.0.id() == e.id() {
                        id = i.2.handle;
                        println!("id{:?}", id);
                        added = true;
                    }
                    // let me: u32 = my_handle.parse().unwrap(); //wrong how do I get my own entity id (!= handle)
                    // let the_id: u32 = i.0.id().into();
                    // println!("me {:?} the_id {:?}", me, the_id);
                    
                    //if entity is me
                    if i.0.id() == me.single_mut().0.id() && added == true{
                        println!("frines hey {:?}", i.2.friends);
                        let mut my_friends = i.2.friends.clone(); 
                        my_friends.insert(id, 88);
                        println!("gogo{:?}", my_friends);
                        // *i.2.friends = *my_friends.as_slice();

                        // let mut new_friends = i.2.friends.clone();
                        // new_friends.insert(id, 0);
                        // *(i.2).friends = new_friends;
                    }
                }
                // do again in case u passed urself before passing friend, do better
                for i in players.iter() {
                    if i.0.id() == e.id() {
                        id = i.2.handle;
                    }
                    let me: u32 = my_handle.parse().unwrap();
                    let the_id: u32 = i.0.id().into();
                    if the_id == me && added == true{
                        println!("frines {:?}", i.2.friends);
                        //i.2.friends.push(id);
                    }
                }
            }
            _ => info!("nothing"),
        }
    }
}
