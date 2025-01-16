use bevy::prelude::*;

pub struct PeoplePlugin; 
impl  Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app  
        .add_systems(Startup, setup)
        .add_systems(Update, print_people_with_jobs)
        .add_systems(Update, print_people_ready_for_employment)
        .add_systems(Update, print_names)
        .add_systems(Update, print_peoples_job);
        
        
    }
    
}

 fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Karl".to_string(),
        },
        Employed {
            job: Job::Developper,
        },
    ));
    commands.spawn((
        Person {
            name: "Loic".to_string(),
        },
        Employed {
            job: Job::Accountant,
        },
    ));
    commands.spawn((
        Person {
            name: "Nathan".to_string(),
        },
        Employed { job: Job::Teacher },
    ));
    commands.spawn(Person {
        name : "Mike".to_string()
    });
}

 fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("My name is {}", person.name); // je sais que je peux utiliser {:?} ou {:#?} mais il faut que je revois leur utilité plus tard.
    }
}
 fn print_people_with_jobs(people: Query<&Person, With<Employed>>) {
    for person in people.iter() {
        println!("{} has a job !", person.name);
    }
}

 fn print_people_ready_for_employment(people: Query<&Person, Without<Employed>>) {
    for person in people.iter() {
        println!("{} is ready to get a job !", person.name);
    }
}

 fn print_peoples_job (people : Query<(&Person, &Employed)>) {
    for (person, employed) in people.iter(){
        let job_title = match employed.job {
            Job::Accountant => "Accountant", 
            Job::Developper => "Developper", 
            Job::Teacher => "Teacher"
        };
        println!("{0} is a {1}", person.name, job_title);
    }
}

#[derive(Component)] //this is a macro
 struct Person {
     name: String,
}
#[derive(Component)]
 struct Employed {
     job: Job,
}
#[derive(Debug)]
 enum Job {
    Developper,
    Accountant,
    Teacher,
}