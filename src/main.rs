use bevy::{ prelude::*, window::PrimaryWindow};
use rand::prelude::*;
pub const PLAYER_SIZE: f32 = 64.;
pub const PLAYER_SPEED: f32 = 500.;
pub const ENEMY_COUNT: usize = 4; 
pub const ENEMY_SIZE: f32 = 64.; 
pub const ENEMY_SPEED: f32 = 480.;
pub const NUMBER_OF_STARS: usize = 10; 
pub const STAR_SIZE: f32 = 30.;
pub const STAR_SPAWN_TIME : f32 = 1.; 
pub const ENEMY_SPAWN_TIME: f32 = 3.;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2, 

}
#[derive(Component)]
pub struct Star {}

#[derive( Resource)]
pub struct Score {
    pub value: u32
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer
}

#[derive(Event)]
pub struct GameOver {
    pub score : u32
}
impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer { timer : Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating)}
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer { timer : Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating)}
    }
}
impl Default for Score {
    fn default() -> Score {
        Score { value : 0}
    }
}


#[derive(Resource)]
pub struct HighScore {
    pub scores: Vec<(String, u32)>
}

impl Default for HighScore {
    fn default() -> HighScore {
        HighScore { scores : Vec::new()}
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<HighScore>()
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, confine_player_mouvement)
        .add_systems(Update, player_mouvement)
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, player_catch_star)
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, tick_enemies_spawn_timer)
        .add_systems(Update, spawn_stars_over_time)
        .add_systems(Update, spawn_enemies_over_time)
        .add_systems(Update, update_high_score)
        .add_systems(Update, exit_game)
        .add_systems(Last, read_high_score)
        .run();
}

fn spawn_camera(mut commands: Commands) {
       commands.spawn((
        Camera2d,
        Camera::default()
    ));
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Sprite::from_image(
        asset_server.load("sprites/ball_blue_large.png"),
    ), Player {}));
}


fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = match window_query.get_single() {
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
        for _ in 0..=ENEMY_COUNT {
            let random_x: f32 = random::<f32>() * window.width()/2.;
            let random_y: f32 = random::<f32>() * window.height()/2.;

            commands.spawn( 
                (Sprite::from_image( asset_server.load("sprites/ball_red_large.png")), 
                Enemy {
                    direction: Vec2::new(random::<f32>(),random::<f32>()).normalize()
                }, 
                Transform::from_xyz(random_x, random_y, 0.)));    
        } 
}

fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>){
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction* ENEMY_SPEED* time.delta_secs();
    }

}
fn update_enemy_direction(mut enemy_query:Query<(&Transform, &mut Enemy)>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = match window_query.get_single(){
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
        for (transform, mut enemy) in enemy_query.iter_mut() {
            let mut direction_changed: bool = false; 
            let half_enemy_size: f32 = ENEMY_SIZE/2.;
            let  translation:Vec3 = transform.translation;  
            let enemy_x_min: f32 =  half_enemy_size - window.width()/2. ;
            let enemy_x_max: f32 = window.width()/2. - half_enemy_size;
            let enemy_y_min: f32 =  half_enemy_size - window.height()/2.;
            let enemy_y_max: f32=   window.height()/2. - half_enemy_size ;

            if  translation.x < enemy_x_min || translation.x > enemy_x_max{ enemy.direction.x *=-1.; direction_changed = true;};
            if  translation.y<enemy_y_min  || translation.y > enemy_y_max {enemy.direction.y *= -1.;direction_changed = true;};

            if direction_changed {}
        }
}

fn player_mouvement( keyboard_input: Res<ButtonInput<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>, time: Res<Time>){
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction = Vec3::ZERO; 
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.,1.,0.)
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::new(1.,0.,0.)
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec3::new(0.,1.,0.)
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.,0.,0.)
        }

        if direction.length() > 0.00 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();

    }
}

pub fn confine_player_mouvement(mut player_query: Query<&mut Transform, With<Player>>, window_query: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut(){
        let window = match window_query.get_single() {
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
        let half_player_size: f32 = PLAYER_SIZE/2.;
        let player_x_min: f32 =  half_player_size - window.width()/2. ;
        let player_x_max: f32 = window.width()/2. - half_player_size;
        let player_y_min: f32 =  half_player_size - window.height()/2.;
        let player_y_max: f32=   window.height()/2. - half_player_size ;

        let mut translation:Vec3 = player_transform.translation; 
        if translation.x < player_x_min { translation.x = player_x_min;}
        else if translation.x > player_x_max {translation.x = player_x_max;}
        if translation.y < player_y_min {translation.y = player_y_min;}
        else if translation.y > player_y_max {translation.y = player_y_max;}
        player_transform.translation = translation; 
    }
}

pub fn spawn_stars(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = match window_query.get_single() {
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
        for _ in 0..=NUMBER_OF_STARS {
            let random_x = random::<f32>() * window.width()/2.;
            let random_y = random::<f32>() * window.height()/2.;

            commands.spawn( 
                (Sprite::from_image( asset_server.load("sprites/star.png")), 
                Star {}, 
                Transform::from_xyz(random_x, random_y, 0.)));
            
        } 
}

pub fn enemy_hit_player(
    mut commands: Commands, 
    mut player_query: Query<(Entity, &Transform), With<Player>>, 
    enemy_query: Query< &Transform, With<Enemy>>,
    mut game_over_event_writer: EventWriter<GameOver>, 
    score: Res<Score>, 
) {
    let impact_distance = (PLAYER_SIZE+ENEMY_SIZE)/2.;
    //add the audio later 
    if let Ok(( player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            if distance <impact_distance {
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver {score : score.value});
            }
        }
    }

}

pub fn player_catch_star (mut commands: Commands, 
    mut player_query: Query<(Entity, &Transform), 
    With<Player>>, 
    star_query: Query<(Entity, &Transform), 
    With<Star>>,
    mut score : ResMut<Score>
) {
    let impact_distance = (STAR_SIZE+PLAYER_SIZE)/2.;
    //add the audio later 
    if let Ok(( _, player_transform)) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform.translation.distance(star_transform.translation);
            if distance <impact_distance {
                commands.entity(star_entity).despawn();
                score.value += 1;
            }
        }
    }
}

pub fn update_score (score: Res<Score>) {
    if score.is_changed() {
        println!("Score : {}", score.value);
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time (
    mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>, star_spawn_timer: Res<StarSpawnTimer>
) {
     let window = match window_query.get_single() {
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
    if star_spawn_timer.timer.finished() {
       let random_x = random::<f32>() * window.width()/2.;
            let random_y = random::<f32>() * window.height()/2.;

            commands.spawn( 
                (Sprite::from_image( asset_server.load("sprites/star.png")), 
                Star {}, 
                Transform::from_xyz(random_x, random_y, 0.)));
            
        } 
    
}

pub fn tick_enemies_spawn_timer (mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>)
{
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>, enemy_spawn_timer: Res<EnemySpawnTimer>) {
     let window = match window_query.get_single() {
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
    if enemy_spawn_timer.timer.finished() {

            let random_x: f32 = random::<f32>() * window.width()/2.;
            let random_y: f32 = random::<f32>() * window.height()/2.;

            commands.spawn( 
                (Sprite::from_image( asset_server.load("sprites/ball_red_large.png")), 
                Enemy {
                    direction: Vec2::new(random::<f32>(),random::<f32>()).normalize()
                }, 
                Transform::from_xyz(random_x, random_y, 0.)));   
            
        } 
}

pub fn exit_game (keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>,  game_over_event: EventReader<GameOver>) {
    if keyboard_input.pressed(KeyCode::Escape)  {app_exit_event_writer.send(AppExit::Success);}
    if !game_over_event.is_empty() {app_exit_event_writer.send(AppExit::Success);}
}

pub fn update_high_score ( mut game_over_event:EventReader<GameOver> , mut high_score: ResMut<HighScore>) {
    for event in  game_over_event.read().into_iter() {
        high_score.scores.push(("Player".to_string(), event.score));
    }
}

pub fn read_high_score (high_score: Res<HighScore>, mut game_over_event:EventReader<GameOver> ) {
for _ in  game_over_event.read().into_iter() {
    high_score.scores.last().inspect(|x|println!("High Scores : {}", x.1))
    
    ;
    }
}
// pub fn init_audio_effects (mut commands, asset_server: Res<AssetServer>)
// {
//     commands.insert_resource(vec![
//         asset_server.load::<AudioSource>(path), 
//     ])
// }