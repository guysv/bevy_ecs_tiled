//! ECS components for Tiled images.
//!
//! This module defines Bevy components used to represent Tiled images within the ECS world.

use bevy::prelude::*;

/// Marker [`Component`] for the [`Sprite`] attached to an image layer.
#[derive(Component, Default, Reflect, Copy, Clone, Debug)]
#[reflect(Component, Default, Debug)]
#[require(Visibility, Transform, Sprite)]
pub struct TiledImage;

/// Component that stores parallax information for Tiled image layers.
#[derive(Component, Reflect, Clone, Debug, Copy)]
#[reflect(Component, Debug)]
pub struct TiledImageParallax {
    /// The horizontal parallax multiplier.
    pub parallax_x: f32,
    /// The vertical parallax multiplier.
    pub parallax_y: f32,
    /// The base position of the image before parallax is applied.
    pub base_position: Vec2,
}

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<TiledImage>();
    app.register_type::<TiledImageParallax>();
    app.add_systems(Update, update_image_parallax);
}

fn update_image_parallax(
    camera_query: Query<&Transform, (With<Camera>, Changed<Transform>)>,
    mut image_query: Query<(&TiledImageParallax, &mut Transform), (With<TiledImage>, Without<Camera>)>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    for (parallax, mut transform) in image_query.iter_mut() {
        let camera_position = Vec2::new(camera_transform.translation.x, camera_transform.translation.y);
        let parallax_offset = Vec2::new(
            camera_position.x * (1.0 - parallax.parallax_x),
            camera_position.y * (1.0 - parallax.parallax_y),
        );

        transform.translation.x = parallax.base_position.x + parallax_offset.x;
        transform.translation.y = parallax.base_position.y + parallax_offset.y;
    }
}
