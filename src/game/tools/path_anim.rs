use crate::*;

//================================-================================-================================
pub struct PathAnimPlugin;
impl Plugin for PathAnimPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_system(update_path_anim);
    }
}

//================================-================================-================================
#[derive(Component)]
pub struct PathAnim {
    path: Vec<(Vec3, f32)>,
    path_index: usize,
}

impl PathAnim {
    pub fn new(
        path: Vec<(Vec3, f32)>,
    ) -> Self {
        Self {
            path: path,
            path_index: 0,
        }
    }

    pub fn set_path(
        &mut self,
        path: Vec<(Vec3, f32)>,
    ) {
        self.path = path;
        self.path_index = 0;
    }

    pub fn is_active(
        &self,
    ) -> bool {
        self.path_index < self.path.len()
    }

    pub fn clear(
        &mut self,
    ) {
        self.path = vec![];
        self.path_index = 0;
    }
}

//================================-================================-================================
fn update_path_anim(
    mut commands: Commands,
    mut path_anim_query: Query<(&Transform, &mut MoveTo, &mut PathAnim)>,
    time: Res<Time>,
) {
    for (transform, mut move_to, mut path_anim) in path_anim_query.iter_mut() {
        if path_anim.is_active() {
            let (target_position, target_speed) = path_anim.path[path_anim.path_index];
            if transform.translation != target_position {
                move_to.position = target_position;
                move_to.speed = target_speed;
            } else {
                path_anim.path_index += 1;
            }
        }
    }
}