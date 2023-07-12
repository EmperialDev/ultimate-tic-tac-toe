use bevy::{prelude::*, window::WindowResized};
use bevy_iced::IcedSettings;

use crate::{CELL_PADDING, CELL_SIZE, GRID_LINE_THICKNESS, TOP_TEXT_SIZE};

pub fn resize(
    mut last_scale_fac: Local<Option<f32>>,
    mut resize_event: EventReader<WindowResized>,
    mut q_scale_factor: Query<&mut ScaleFactor>,
    mut q_scale: Query<&mut Transform, (With<Scale>, Without<TextScale>)>,
    mut q_text_scale: Query<(&mut Transform, &mut Text), With<TextScale>>,
    mut q_ui_scale: Query<&mut Style, With<UiScale>>,
    mut iced_settings: ResMut<IcedSettings>,
) {
    for event in resize_event.iter() {
        let scale_num_x =
            9.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) + 2.5 * TOP_TEXT_SIZE;
        let scale_num_y =
            9.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) + 4.0 * TOP_TEXT_SIZE;

        let scale_x = event.height / scale_num_x;
        let scale_y = event.width / scale_num_y;

        let scale_fac = if scale_x < scale_y { scale_x } else { scale_y };

        q_scale_factor.single_mut().0 = scale_fac;

        let scale_fac_diffrens = if let Some(last_scale_fac) = *last_scale_fac {
            scale_fac / last_scale_fac
        } else {
            scale_fac
        };

        *last_scale_fac = Some(scale_fac);

        if iced_settings.scale_factor.is_some() {
            iced_settings.scale_factor = Some((scale_fac * 3.0) as f64)
        }

        for mut transform in &mut q_scale {
            transform.translation *= scale_fac_diffrens;
            transform.scale *= scale_fac_diffrens;
        }

        for (mut transform, mut text) in &mut q_text_scale {
            transform.translation *= scale_fac_diffrens;
            text.sections[0].style.font_size *= scale_fac_diffrens;
        }

        for mut style in &mut q_ui_scale {
            // Size
            if let Val::Px(_) = style.size.width {
                style.size.width *= scale_fac_diffrens;
            }
            if let Val::Px(_) = style.size.height {
                style.size.height *= scale_fac_diffrens;
            }
            // Padding
            if let Val::Px(_) = style.padding.bottom {
                style.padding.bottom *= scale_fac_diffrens;
            }
            if let Val::Px(_) = style.padding.left {
                style.padding.left *= scale_fac_diffrens;
            }
            if let Val::Px(_) = style.padding.right {
                style.padding.right *= scale_fac_diffrens;
            }
            if let Val::Px(_) = style.padding.top {
                style.padding.top *= scale_fac_diffrens;
            }
        }
    }
}

#[derive(Component)]
pub struct Scale;

#[derive(Component)]
pub struct TextScale;

#[derive(Component)]
pub struct UiScale;

#[derive(Component, Default)]
pub struct ScaleFactor(pub f32);
