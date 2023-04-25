use leafwing_input_manager::orientation::{Orientation, Rotation};

use crate::*;

//================================-================================-================================
pub struct MoveToPlugin;
impl Plugin for MoveToPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_system(update_move_to);
    }
}

//================================-================================-================================
#[derive(Component)]
pub struct MoveTo {
    pub look_axis: Vec3,
    pub look_direction: Vec3,
    pub position: Vec3,
    pub speed: f32,
}

impl MoveTo {
    pub fn new(
        look_direction: &Vec3,
        position: &Vec3,
        speed: f32,
    ) -> Self {
        Self {
            look_axis: Vec3::Y,
            look_direction: *look_direction,
            position: *position,
            speed,
        }
    }

    pub fn with_look_axis(
        mut self,
        look_axis: &Vec3,
    ) -> Self {
        self.look_axis = *look_axis;
        self
    }
}

//================================-================================-================================
fn update_move_to(
    mut commands: Commands,
    mut transform_query: Query<(&mut Transform, &MoveTo)>,
    time: Res<Time>,
) {
    let mut rng = thread_rng();
    for (mut transform, move_to) in transform_query.iter_mut() {
        let look_at = transform.translation + move_to.look_direction;
        transform.look_at(look_at, move_to.look_axis);

        if transform.translation != move_to.position {
            let distance_vector = move_to.position - transform.translation;
            let move_vector = distance_vector.normalize() * move_to.speed * time.delta_seconds();
            if move_vector.length() > distance_vector.length() {
                transform.translation = move_to.position;
            } else {
                transform.translation += move_vector;
            }
        }
    }
}