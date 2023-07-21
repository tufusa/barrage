use std::f32::consts::PI;

use bevy::prelude::*;

pub(crate) fn sync<Parent: Component>(
    enemy_query: Query<(Entity, &Children), With<Parent>>,
    mut transform_query: Query<&mut Transform>,
) {
    enemy_query.iter().for_each(|(parent, children)| {
        let parent_transform: Transform;
        if let Ok(transform) = transform_query.get(parent) {
            parent_transform = *transform;
        } else {
            return;
        }

        children.iter().for_each(|child| {
            if let Ok(mut child_transform) = transform_query.get_mut(*child) {
                child_transform.translation = parent_transform.translation;
                child_transform.rotation = parent_transform.rotation;
            }
        });
    });
}
