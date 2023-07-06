use bevy::{prelude::*, window::WindowResized};

use crate::{CELL_PADDING, CELL_SIZE, GRID_LINE_THICKNESS, TEXT_SIZE};

pub fn resize(
    mut resize_event: EventReader<WindowResized>,
    mut q_scale_factor: Query<&mut ScaleFactor>,
    mut q_better_scale: Query<(&mut Transform, &BetterScale), Without<TextScale>>,
    mut q_text_scale: Query<(&mut Transform, &mut Text, &TextScale), Without<BetterScale>>,
) {
    for event in resize_event.iter() {
        let scale_num_x =
            9.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) + 2.5 * TEXT_SIZE;
        let scale_num_y =
            9.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) + 4.0 * TEXT_SIZE;

        let scale_x = event.height / scale_num_x;
        let scale_y = event.width / scale_num_y;

        let scale_fac = if scale_x < scale_y { scale_x } else { scale_y };

        q_scale_factor.single_mut().0 = scale_fac;

        for (mut transform, scale) in &mut q_better_scale {
            transform.translation = scale.translation * scale_fac;
            transform.scale = scale.scale * scale_fac;
        }

        for (mut transform, mut text, scale) in &mut q_text_scale {
            transform.translation = scale.translation * scale_fac;
            text.sections[0].style.font_size = scale.text_size * scale_fac;
        }
    }
}

#[derive(Component)]
pub struct BetterScale {
    pub translation: Vec3,
    pub scale: Vec3,
}

impl BetterScale {
    /// Creates a new `BetterScale` from the `translation` and `scale`
    pub fn new(translation: Vec3, scale: Vec3) -> Self {
        BetterScale { translation, scale }
    }

    /// Creates a new `BetterScale` from the `translation`
    pub fn from_location(translation: Vec3) -> Self {
        Self::new(translation, Vec3::splat(1.0))
    }

    /// Creates a new `BetterScale` from the `scale`
    pub fn from_scale(scale: Vec3) -> Self {
        Self::new(Vec3::splat(0.0), scale)
    }
}

#[derive(Component)]
pub struct TextScale {
    pub translation: Vec3,
    pub text_size: f32,
}

impl TextScale {
    /// Creates a new `TextScale` from the `translation` and `text_size`
    pub fn new(translation: Vec3, text_size: f32) -> Self {
        TextScale {
            translation,
            text_size,
        }
    }
}

#[derive(Component, Default)]
pub struct ScaleFactor(pub f32);
