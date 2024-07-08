use bevy::{
    prelude::*,
};

use bevy_water::*;

#[derive(Bundle, Default)]
struct ShipBundle {
  ship: Ship,
  name: Name,
  spatial: SpatialBundle,
}

#[derive(Component, Default, Clone)]
pub struct Ship {
  water_line: f32,
  front: Vec3,
  back_left: Vec3,
  back_right: Vec3,
}

impl Ship {
  fn new(water_line: f32, front: f32, back: f32, left: f32, right: f32) -> Self {
    Self {
      water_line,
      front: Vec3::new(0.0, 0.0, front),
      back_left: Vec3::new(left, 0.0, back),
      back_right: Vec3::new(right, 0.0, back),
    }
  }

  fn update(
    &self,
    water: &WaterParam,
    pos: Vec3,
    transform: &mut Transform,
  ) {
    let (yaw, _pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
    let global = Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(yaw));

    // Get the wave position at the front, back_left and back_right.
    let mut front = water.wave_point(global.transform_point(self.front));
    let left = water.wave_point(global.transform_point(self.back_left));
    let right = water.wave_point(global.transform_point(self.back_right));
    let normal = (left - front).cross(right - front).normalize();

    front.y += self.water_line - 0.2;
    transform.look_at(front, normal);

    transform.translation.y = ((front.y + left.y + right.y) / 3.0) + self.water_line;
  }
}

pub fn update_ships(
  water: WaterParam,
  mut ships: Query<(&Ship, &mut Transform, &GlobalTransform)>,
) {
  for (ship, mut transform, global) in ships.iter_mut() {
    let pos = global.translation();
    ship.update(&water, pos, &mut transform);
  }
}

pub fn spawn_ships(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
  // Spawn ships.
  let scene = asset_server.load("models/dutch_ship_medium_1k/dutch_ship_medium_1k.gltf#Scene0");
  let ship = Ship::new(-0.400, -8.0, 9.0, -2.0, 2.0);

  let mut spawn_ship = |name: &str, transform: Transform| {
    commands
      .spawn(ShipBundle {
        ship: ship.clone(),
        name: Name::new(name.to_string()),
        spatial: SpatialBundle {
          transform,
          ..default()
        },
        ..default()
      })
      .with_children(|parent| {
        parent.spawn(SceneBundle {
          scene: scene.clone(),
          // Rotate ship model to line up with rotation axis.
          transform: Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
          ..default()
        });
      });
  };

  // One ship in the harbour.
  spawn_ship("Ship 1", Transform::from_xyz(35.0, 0.0, 2.0)
      .with_rotation(Quat::from_rotation_y(2.4)));
  spawn_ship("Ship 2", Transform::from_xyz(180.0, 0.0, -113.0)
      .with_rotation(Quat::from_rotation_y(4.8)));
}
