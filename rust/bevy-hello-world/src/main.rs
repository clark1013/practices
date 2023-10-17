use bevy::prelude::*;
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn hello_world() {
    println!("hello world")
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Ball;

fn setup(mut commands: Commands) {
    commands.spawn((Person, Name("A".to_string())));
    commands.spawn((Person, Name("B".to_string())));
    commands.spawn((Person, Name("C".to_string())));
}

fn greet_pepole(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}", name.0)
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (hello_world, greet_pepole))
        .run()
}
