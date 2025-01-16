use bevy::prelude::*;
mod people_plugin; 

fn main() {
    App::new()
        .add_plugins(people_plugin::PeoplePlugin)
    .run();
      
}


