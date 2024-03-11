use bevy::prelude::*;

pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, add_motion).add_systems(Last, update_motion);
    }
}

#[derive(Debug, Copy, Clone, Component)]
pub struct Motion {
    motion: Vec3,
    last_position: Vec3,
}

impl Motion {
    pub fn motion(&self) -> Vec3 {
        self.motion
    }
}

fn add_motion(mut commands: Commands, q: Query<(Entity, &GlobalTransform), Added<GlobalTransform>>) {
    for (entity, transform) in &q {
        commands.entity(entity).insert(Motion {
            motion: Vec3::ZERO,
            last_position: transform.translation(),
        });
    }
}

fn update_motion(time: Res<Time>, mut q: Query<(&mut Motion, &GlobalTransform)>) {
    let dt = time.delta_seconds();
    let first_frame = dt < 1e-6;
    for (mut motion, transform) in &mut q {
        let cur_pos = transform.translation();
        let delta = cur_pos - motion.last_position;
        if !first_frame {
            motion.motion = delta / dt;
        }
        motion.last_position = cur_pos;
    }
}