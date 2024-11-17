### **Step-by-Step Guide to Create a 2D Pong Game Using Rust and Bevy**

Bevy is a game engine built in Rust that prioritizes simplicity and performance. In this guide, we’ll walk through creating a 2D Pong game using Bevy, starting from setting up the environment to implementing core mechanics like player movement and scoring.

---

### **1. Setting Up the Project**
Start by creating a new Rust project with Bevy:

```bash
cargo new pong_game
cd pong_game
```

Edit your `Cargo.toml` to include the necessary dependencies:

```toml
[dependencies]
bevy = "0.14"
rand = "*"
bevy_rapier2d = "*"
```

---

### **2. Initialize Bevy and Configure the Window**
Create a main function that initializes the Bevy app. Define constants for the window size and paddle dimensions to simplify future calculations:

```rust
const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HEIGHT: f32 = 720.;
```

Inside `main.rs`, configure the Bevy app:

```rust
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}
```

This initializes a window with the desired resolution and includes the Rapier plugin for physics simulation.

---

### **3. Add Game Components and Resources**
Define the core game components, such as paddles and the ball. Use `#[derive(Component)]` to create reusable structs.

#### **Paddle and Player Components:**
```rust
#[derive(Component)]
struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
    Player1,
    Player2,
}
```

Each paddle has specific keys assigned for movement, and players are differentiated using the `Player` enum.

#### **Resource for Scores:**
```rust
#[derive(Default, Resource)]
struct Score(HashMap<Player, i32>);
```

---

### **4. Set Up the Game Scene**
#### **Spawn Camera:**
Add a 2D camera to the game:

```rust
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
```

#### **Spawn Paddles:**
Define paddles with unique controls and colors for each player:

```rust
fn spawn_players(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2. + 20., 0., 0.)),
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(10., 150.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::W,
            move_down: KeyCode::S,
        },
        Player::Player1,
        RigidBody::KinematicPositionBased,
        Collider::cuboid(5., 75.),
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2. - 20., 0., 0.)),
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(10., 150.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
        },
        Player::Player2,
        RigidBody::KinematicPositionBased,
        Collider::cuboid(5., 75.),
    ));
}
```

#### **Spawn Ball:**
Create a dynamic ball with a collider:

```rust
fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(50., 50.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Ball,
        RigidBody::Dynamic,
        Collider::ball(25.),
        Velocity::linear(Vec2::new(100., 0.)),
        Restitution::coefficient(1.1),
    ));
}
```

---

### **5. Implement Movement Systems**
#### **Paddle Movement:**
Allow paddles to move up and down based on player input:

```rust
fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) {
            pos.translation.y += 500. * time.delta_seconds();
        }
        if input.pressed(settings.move_down) {
            pos.translation.y -= 500. * time.delta_seconds();
        }
    }
}
```

---

### **6. Handle Collisions and Scoring**
#### **Ball and Paddle Collision:**
Detect when the ball hits a paddle and change its color based on the player:

```rust
fn ball_hit(
    paddles: Query<&Player, With<Paddle>>,
    mut balls: Query<(&CollidingEntities, &mut Sprite), With<Ball>>,
) {
    for (hits, mut sprite) in &mut balls {
        for hit in hits.iter() {
            if let Ok(player) = paddles.get(hit) {
                sprite.color = match player {
                    Player::Player1 => Color::RED,
                    Player::Player2 => Color::GREEN,
                };
            }
        }
    }
}
```

#### **Scoring System:**
Increment player scores when the ball hits the goal area:

```rust
fn score(
    mut events: EventReader<GameEvents>,
    mut score: ResMut<Score>,
    mut score_text: Query<(&mut Text, &Player)>,
) {
    for event in events.read() {
        if let GameEvents::GainPoint(player) = event {
            *score.0.entry(*player).or_default() += 1;
            let player_score = score.0.get(player).unwrap();
            for (mut text, owner) in &mut score_text {
                if owner == player {
                    text.sections[0].value = player_score.to_string();
                }
            }
        }
    }
}
```

---

### **7. Adding Debugging Tools**
Use `bevy_rapier2d`'s debug plugin for real-time collision visualization:

```rust
#[cfg(debug_assertions)]
app.add_plugins(RapierDebugRenderPlugin::default());
```

---

### **8. Running the Game**
Run the game with:

```bash
cargo run
```

You now have a fully functional Pong game with two players, score tracking, and collision handling!

---

### **Next Steps**
- Add audio for ball collisions.
- Enhance the UI with better score displays.
- Implement AI for single-player mode. 

This project is a great introduction to Bevy and physics-based games!