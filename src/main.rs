use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup)
            .add_system(print_names)
            .add_system(print_job);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(
        (
            Person {
                name: "John".to_string()
            },
            Employed {
                job: Job::Programmer
            }),
    );

    commands.spawn(
        (
            Person {
                name: "Jane".to_string()
            },
            Employed {
                job: Job::Artist
            }),
    );

    commands.spawn(
        (
            Person {
                name: "Bob".to_string()
            },
            Employed {
                job: Job::Designer
            }),
    );

    commands.spawn(
        Person {
            name: "Alice".to_string()
        });
}

pub fn print_names(query: Query<&Person>) {
    for person in query.iter() {
        println!("Person: {}", person.name);
    }
}

pub fn print_job(query: Query<(&Person, &Employed)>) {
    for (person, employed) in query.iter() {
        let job = match employed.job {
            Job::Programmer => "Programmer",
            Job::Artist => "Artist",
            Job::Designer => "Designer",
            Job::Manager => "Manager",
        };

        println!("{} is a {}", person.name, job);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Programmer,
    Artist,
    Designer,
    Manager,
}