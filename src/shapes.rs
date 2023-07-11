use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn generate_cross_path(size: f32, thickness: f32) -> Path {
    let center_to_inner_cross = (std::f32::consts::SQRT_2 * thickness) / 2.0;
    let half_size = size / 2.0;

    let math1 = thickness - center_to_inner_cross;
    let math2 = (std::f32::consts::SQRT_2 * math1) / 2.0;
    let math3 = (std::f32::consts::SQRT_2 * math2) / 2.0;

    let outer_corner = half_size - center_to_inner_cross;

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::new(0.0, center_to_inner_cross));
    path_builder.line_to(Vec2::new(outer_corner - math3, half_size - math3));

    path_builder.arc(
        Vec2::splat(half_size - thickness / 2.0),
        Vec2::splat(thickness / 2.0),
        -std::f32::consts::PI,
        0.0,
    );

    path_builder.line_to(Vec2::new(center_to_inner_cross, 0.0));
    path_builder.line_to(Vec2::new(half_size - math3, -outer_corner + math3));

    path_builder.arc(
        Vec2::new(half_size - thickness / 2.0, -half_size + thickness / 2.0),
        Vec2::splat(thickness / 2.0),
        -std::f32::consts::PI,
        0.0,
    );

    path_builder.line_to(Vec2::new(0.0, -center_to_inner_cross));
    path_builder.line_to(Vec2::new(-outer_corner + math3, -half_size + math3));

    path_builder.arc(
        Vec2::new(-half_size + thickness / 2.0, -half_size + thickness / 2.0),
        Vec2::splat(thickness / 2.0),
        -std::f32::consts::PI,
        0.0,
    );

    path_builder.line_to(Vec2::new(-center_to_inner_cross, 0.0));
    path_builder.line_to(Vec2::new(-half_size + math3, outer_corner - math3));

    path_builder.arc(
        Vec2::new(-half_size + thickness / 2.0, half_size - thickness / 2.0),
        Vec2::splat(thickness / 2.0),
        -std::f32::consts::PI,
        0.0,
    );

    path_builder.close();

    path_builder.build()
}

pub fn generate_nought_path(size: f32, thickness: f32) -> Path {
    let mut path_builder = PathBuilder::new();

    path_builder.move_to(Vec2::new(size / 2.0, 0.0));
    path_builder.arc(
        Vec2::splat(0.0),
        Vec2::splat(size / 2.0),
        2.0 * std::f32::consts::PI,
        0.0,
    );
    path_builder.move_to(Vec2::new(size / 2.0 - thickness, 0.0));
    path_builder.arc(
        Vec2::splat(0.0),
        Vec2::splat(size / 2.0 - thickness),
        2.0 * std::f32::consts::PI,
        0.0,
    );

    path_builder.build()
}
