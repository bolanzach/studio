use bevy::prelude::*;

use crate::Tag;

pub fn paint_system(
    query: Query<(&Tag, &Handle<Mesh>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (tag, handle) in query.iter() {
        let mesh = handle.id();
    }
}
