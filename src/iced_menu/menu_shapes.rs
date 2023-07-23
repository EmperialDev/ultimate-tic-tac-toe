use iced::{
    widget::canvas::{path::*, Path},
    Point,
};

pub fn generate_cross_path(center: Point, size: f32, thickness: f32) -> Path {
    let center_to_inner_cross = (std::f32::consts::SQRT_2 * thickness) / 2.0;
    let half_size = size / 2.0;

    let math1 = thickness - center_to_inner_cross;
    let math2 = (std::f32::consts::SQRT_2 * math1) / 2.0;
    let math3 = (std::f32::consts::SQRT_2 * math2) / 2.0;

    let outer_corner = half_size - center_to_inner_cross;

    Path::new(|b| {
        b.move_to(Point::new(0.0, center_to_inner_cross).add(center));
        b.line_to(Point::new(outer_corner - math3, half_size - math3).add(center));
        b.line_to(Point::new(half_size - math3, outer_corner - math3).add(center));

        b.line_to(Point::new(center_to_inner_cross, 0.0).add(center));
        b.line_to(Point::new(half_size - math3, -outer_corner + math3).add(center));
        b.line_to(Point::new(outer_corner - math3, -half_size + math3).add(center));

        b.line_to(Point::new(0.0, -center_to_inner_cross).add(center));
        b.line_to(Point::new(-outer_corner + math3, -half_size + math3).add(center));
        b.line_to(Point::new(-half_size + math3, -outer_corner + math3).add(center));

        b.line_to(Point::new(-center_to_inner_cross, 0.0).add(center));
        b.line_to(Point::new(-half_size + math3, outer_corner - math3).add(center));
        b.line_to(Point::new(-outer_corner + math3, half_size - math3).add(center));

        b.close();

        let arc = Arc {
            center: Point::new(half_size - thickness / 2.0, half_size - thickness / 2.0)
                .add(center),
            radius: thickness / 2.0,
            start_angle: 135.0f32.to_radians(),
            end_angle: -45.0f32.to_radians(),
        };

        b.arc(arc);
        b.close();

        let arc = Arc {
            center: Point::new(half_size - thickness / 2.0, -half_size + thickness / 2.0)
                .add(center),
            radius: thickness / 2.0,
            start_angle: 45.0f32.to_radians(),
            end_angle: -135.0f32.to_radians(),
        };

        b.arc(arc);
        b.close();

        let arc = Arc {
            center: Point::new(-half_size + thickness / 2.0, -half_size + thickness / 2.0)
                .add(center),
            radius: thickness / 2.0,
            start_angle: -45.0f32.to_radians(),
            end_angle: -225.0f32.to_radians(),
        };

        b.arc(arc);
        b.close();

        let arc = Arc {
            center: Point::new(-half_size + thickness / 2.0, half_size - thickness / 2.0)
                .add(center),
            radius: thickness / 2.0,
            start_angle: -135.0f32.to_radians(),
            end_angle: -315.0f32.to_radians(),
        };

        b.arc(arc);
        b.close();
    })
}

pub fn generate_nought_path(center: Point, size: f32, thickness: f32) -> Path {
    Path::new(|b| {
        b.move_to(Point::new(size / 2.0, 0.0).add(center));

        let arc = Arc {
            center,
            radius: size / 2.0,
            start_angle: 360.0,
            end_angle: 0.0,
        };

        b.arc(arc);
        b.move_to(Point::new(size / 2.0 - thickness, 0.0).add(center));

        let arc = Arc {
            center,
            radius: size / 2.0 - thickness,
            start_angle: -360.0,
            end_angle: 0.0,
        };

        b.arc(arc);
    })
}

trait Add {
    fn add(self, v: Self) -> Self;
}

impl Add for Point {
    fn add(self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}
