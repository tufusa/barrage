use bevy::prelude::*;

pub(crate) fn sync<Parent: Component>(
    enemy_query: Query<(Entity, &Children), With<Parent>>,
    mut transform_query: Query<&mut Transform>,
) {
    enemy_query.iter().for_each(|(parent, children)| {
        let translation: Vec3;

        if let Ok(parent_transform) = transform_query.get(parent) {
            translation = parent_transform.translation;
        } else {
            return;
        };

        children.iter().for_each(|child| {
            if let Ok(mut child_transform) = transform_query.get_mut(*child) {
                child_transform.translation = translation;
            }
        })
    });
}
