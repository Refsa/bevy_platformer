use bevy::prelude::*;

pub struct HelloPlugin;

struct Person;
struct Name(String);
struct GreetTimer(Timer);

fn hello_world() {
    println!("Hello World!");
}

fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("One".to_string())))
        .spawn((Person, Name("Two".to_string())))
        .spawn((Person, Name("Three".to_string())));
}

fn greet_people_system(_time: Res<Time>, mut timer: ResMut<GreetTimer>, _person: &Person, name: &Name) {
    timer.0.tick(_time.delta_seconds);

    if timer.0.finished {
        println!("Hello, {}!", name.0);
    }
}

fn greet_people_query(_time: Res<Time>, mut timer: ResMut<GreetTimer>, mut query: Query<(&Person, &Name)>) {
    timer.0.tick(_time.delta_seconds);

    if timer.0.finished {
        for (_person, _name) in &mut query.iter() {
            println!("Hello, {}!", _name.0);
        }
    }
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people_query.system());
    }
}
