pub mod body;
pub mod camera;
pub mod canvas;
pub mod color;
pub mod computed_intersection;
pub mod cube;
pub mod cylinder;
pub mod double_cone;
pub mod group;
pub mod intersections;
pub mod material;
pub mod matrix;
pub mod pattern;
pub mod plane;
pub mod point_light;
pub mod ray;
pub mod sphere;
pub mod tuple;
pub mod world;

#[allow(dead_code)]
pub mod consts {
    pub(crate) const EPSILON: f64 = 0.00001;
    pub const PI: f64 = std::f64::consts::PI;
    pub const PI_BY_2: f64 = std::f64::consts::FRAC_PI_2;
    pub const PI_BY_3: f64 = std::f64::consts::FRAC_PI_3;
    pub const PI_BY_4: f64 = std::f64::consts::FRAC_PI_4;
    pub const PI_BY_6: f64 = std::f64::consts::FRAC_PI_6;
    pub const SQRT_2: f64 = std::f64::consts::SQRT_2;
}

pub trait RoundToNDecimalPlaces {
    fn round_to_n_decimal_places(&self, digits: i32) -> Self;
}

impl RoundToNDecimalPlaces for f64 {
    /// ```
    /// use raytracer_rust::RoundToNDecimalPlaces;
    /// assert_eq!(0.009.round_to_n_decimal_places(2), 0.01);
    /// ```
    fn round_to_n_decimal_places(&self, digits: i32) -> Self {
        let ten_n = (10.0_f64).powi(digits);
        (self * ten_n).round() / ten_n
    }
}

#[macro_export]
macro_rules! max {
    ($a:expr, $b:expr) => {{
        if ($a > $b) { $a } else { $b }
    }};

    ($a:expr, $($b:expr),+ $(,)?) => {{
        max!($a,max!($($b),+))
    }}
}

#[macro_export]
macro_rules! min {
    ($a:expr, $b:expr) => {{
        if ($b > $a) { $a } else { $b }
    }};

    ($a:expr, $($b:expr),+ $(,)?) => {{
        min!($a,min!($($b),+))
    }}
}

use crate::{
    body::{Body, Intersectable},
    camera::Camera,
    canvas::{Canvas, ToPPM},
    color::Color,
    consts::{PI, PI_BY_2, PI_BY_3, PI_BY_4, PI_BY_6},
    cube::Cube,
    cylinder::Cylinder,
    double_cone::DoubleCone,
    group::Group,
    material::{Material, Phong, PhongLighting},
    matrix::Matrix,
    pattern::{Checkers, Flat, Gradient, Pattern, Ring, Striped},
    plane::Plane,
    point_light::PointLight,
    ray::Ray,
    sphere::Sphere,
    tuple::Tuple,
    world::World,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{sync::Mutex, time::Instant};

pub fn chapter6_challenge() {
    println!("Chapter 6 challenge without multi-threading ...");
    let now = Instant::now();
    let width: usize = 800;
    let mut canvas = Canvas::new(width, width);
    let mut s = Sphere::default();
    match s.material_mut() {
        // Material::Phong(p) => p.color = Color::new(1.0, 0.2, 1.0),
        Material::Phong(p) => p.pattern = Pattern::Flat(Flat::new(Color::new(1.0, 0.2, 1.0))),
    }
    let cw: usize = width;
    let size = 10f64;
    let half = size / 2.0;
    (0..cw).into_iter().for_each(|j| {
        (0..cw).into_iter().for_each(|i| {
            let x = -(half as f64) + (i as f64 / cw as f64) * size;
            let y = (half as f64) - (j as f64 / cw as f64) * size;
            let light =
                PointLight::new(Tuple::Point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
            let point = Tuple::Point(x, y, 10.0);
            let ray = Ray::new(
                Tuple::Point(0.0, 0.0, -5.0),
                (point - Tuple::Point(0.0, 0.0, -5.0)).normalize(),
            );

            let xs = s.intersect(&ray);

            if let Some(hit) = xs.hit() {
                let intersection_point = ray.position(hit.t);
                let normal = hit.body.normal_at(intersection_point);
                let eye = -(ray.direction);
                let color = hit
                    .body
                    .material()
                    .lighting(&hit.body, light, point, eye, normal, 0.0);

                canvas.set_color_at_pixel(i, j, color);
            }
        })
    });

    // CHapter 6 WithOut Threads
    canvas.save_as_ppm("challenges/ch6.ppm").unwrap();
    let elapsed = now.elapsed();
    println!("time taken: {} ms", elapsed.as_millis());
}

// FIXME: This multithreading implementation is not totally correct,
// impl Rayon traits for Canvas struct to improve performance.
pub fn chapter6_challenge_parallel() {
    println!("Chapter 6 challenge with multi-threading ...");
    let now = Instant::now();
    let width: usize = 800;
    let world = Mutex::new(Canvas::new(width, width));
    let mut s = Sphere::default();
    match s.material_mut() {
        // Material::Phong(p) => p.color = Color::new(1.0, 0.2, 1.0),
        Material::Phong(p) => p.pattern = Pattern::Flat(Flat::new(Color::new(1.0, 0.2, 1.0))),
    }
    let cw: usize = width;
    let size = 10f64;
    let half = size / 2.0;
    (0..cw).into_par_iter().for_each(|j| {
        (0..cw).into_par_iter().for_each(|i| {
            let x = -(half as f64) + (i as f64 / cw as f64) * size;
            let y = (half as f64) - (j as f64 / cw as f64) * size;
            let light =
                PointLight::new(Tuple::Point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
            let point = Tuple::Point(x, y, 10.0);
            let ray = Ray::new(
                Tuple::Point(0.0, 0.0, -5.0),
                (point - Tuple::Point(0.0, 0.0, -5.0)).normalize(),
            );

            let xs = s.intersect(&ray);

            if let Some(hit) = xs.hit() {
                let intersection_point = ray.position(hit.t);
                let normal = hit.body.normal_at(intersection_point);
                let eye = -(ray.direction);
                let color = hit
                    .body
                    .material()
                    .lighting(&hit.body, light, point, eye, normal, 0.0);

                // Introduced lock in another scope to unlock the variable just after completion of this command
                // and not wait until the whole block to complete execution
                // *No difference here as this lock statement is the last command.
                // *Good to know!
                {
                    world.lock().unwrap().set_color_at_pixel(i, j, color);
                }
            }
        })
    });

    world
        .lock()
        .unwrap()
        .save_as_ppm("challenges/ch6.ppm")
        .unwrap();
    let elapsed = now.elapsed();
    println!("time taken: {} ms", elapsed.as_millis());
}

fn chapter7_setup() -> (World, Camera) {
    let floor = Sphere::default()
        .with_transform(Matrix::Scaling(10.0, 0.01, 10.0))
        .with_material(Material::Phong(Phong {
            // color: Color::new(1.0, 0.9, 0.9),
            pattern: pattern::Pattern::Flat(Flat::new(Color::new(1.0, 0.9, 0.9))),
            specular: 0.0,
            ..Default::default()
        }));

    let left_wall = Sphere::default()
        .with_transform(
            Matrix::Translation(0, 0, 5)
                * Matrix::rotation_Y(-consts::PI_BY_4)
                * Matrix::rotation_X(consts::PI_BY_2)
                * Matrix::Scaling(10.0, 0.01, 10.0),
        )
        .with_material(*floor.material());

    let right_wall = Sphere::default()
        .with_transform(
            Matrix::Translation(0, 0, 5)
                * Matrix::rotation_Y(consts::PI_BY_4)
                * Matrix::rotation_X(consts::PI_BY_2)
                * Matrix::Scaling(10.0, 0.01, 10.0),
        )
        .with_material(*floor.material());

    let middle = Sphere::default()
        .with_transform(Matrix::Translation(-0.5, 1.0, 0.5))
        .with_material(Material::Phong(Phong {
            // color: Color::new(0.1, 1.0, 0.5),
            pattern: pattern::Pattern::Flat(Flat::new(Color::new(0.1, 1.0, 0.5))),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        }));

    let right = Sphere::default()
        .with_transform(Matrix::Translation(1.5, 0.5, -0.5) * Matrix::Scaling(0.5, 0.5, 0.5))
        .with_material(Material::Phong(Phong {
            // color: Color::new(0.5, 1.0, 0.1),
            pattern: pattern::Pattern::Flat(Flat::new(Color::new(0.5, 1.0, 0.1))),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        }));

    let left = Sphere::default()
        .with_transform(Matrix::Translation(-1.5, 0.33, -0.75) * Matrix::Scaling(0.33, 0.33, 0.33))
        .with_material(Material::Phong(Phong {
            // color: Color::new(1.0, 0.8, 0.1),
            pattern: pattern::Pattern::Flat(Flat::new(Color::new(1.0, 0.8, 0.1))),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        }));

    let mut world = World::default();
    world.add_point_light(PointLight::new(
        Tuple::Point(-10, 10, -10),
        Color::new(1.0, 1.0, 1.0),
    ));
    world.add_body(floor.into());
    world.add_body(left_wall.into());
    world.add_body(right_wall.into());
    world.add_body(middle.into());
    world.add_body(right.into());
    world.add_body(left.into());

    let camera = Camera::new(1200, 600, PI_BY_3).look_at_from_position(
        Tuple::Point(0.0, 1.5, -5.0),
        Tuple::Point(0, 1, 0),
        Tuple::Vector(0, 1, 0),
    );

    (world, camera)
}

pub fn chapter7_challenge() {
    println!("Chapter 7 challenge without multi-threading ...");
    let now = Instant::now();
    let (world, camera) = chapter7_setup();
    camera
        .render(&world)
        .save_as_ppm("challenges/ch7.ppm")
        .unwrap();
    let elapsed = now.elapsed();
    println!("time taken: {} ms", elapsed.as_millis());
}

pub fn chapter7_challenge_parallel() {
    println!("Chapter 7 challenge with multi-threading ...");
    let now = Instant::now();
    let (world, camera) = chapter7_setup();
    camera
        .render_par(&world)
        .save_as_ppm("challenges/ch7.ppm")
        .unwrap();
    let elapsed = now.elapsed();
    println!("time taken: {} ms", elapsed.as_millis());
}

pub fn chapter9_challenge() {
    println!("Chapter 9 challenge with multi-threading ...");
    let now = Instant::now();

    let light = PointLight::new(Tuple::Point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    // Floor
    let floor_material = Phong {
        pattern: Pattern::Flat(Flat::new(Color::new(0.5, 0.45, 0.45))),
        specular: 0.0,
        ..Default::default()
    };

    let floor = Plane::default().with_material(Material::from(floor_material));

    // Spheres
    let left_sphere = Sphere::new(
        Matrix::Translation(-1.5, 0.33, -0.75) * Matrix::Scaling(0.33, 0.33, 0.33),
        Material::Phong(Phong {
            pattern: Pattern::Flat(Flat::new(Color::new(0.78, 0.28, 0.96))),
            ..Default::default()
        }),
    );

    let middle_sphere = Sphere::new(
        Matrix::Translation(-0.5, 1.0, 0.5),
        Material::Phong(Phong {
            pattern: Pattern::Flat(Flat::new(Color::new(1.0, 0.49, 0.0))),
            diffuse: 0.7,
            specular: 0.1,
            shininess: 50.0,
            ..Default::default()
        }),
    );

    let right_sphere = Sphere::new(
        Matrix::Translation(1.5, 0.5, -0.5) * Matrix::Scaling(0.5, 0.5, 0.5),
        Material::Phong(Phong {
            pattern: Pattern::Flat(Flat::new(Color::new(0.51, 0.75, 0.06))),
            ..Default::default()
        }),
    );

    let world = World::new(
        vec![light],
        vec![
            Body::from(floor),
            Body::from(left_sphere),
            Body::from(middle_sphere),
            Body::from(right_sphere),
        ],
        vec![],
        0,
    );

    let camera = Camera::new(800, 800, PI_BY_3).look_at_from_position(
        Tuple::Point(0.0, 2.3, -8.0),
        Tuple::Point(0.0, 1.0, 0.0),
        Tuple::Vector(0.0, 1.0, 0.0),
    );

    camera
        .render_par(&world)
        .save_as_ppm("challenges/ch9.ppm")
        .unwrap();
    let elapsed = now.elapsed();

    println!("time taken: {} ms", elapsed.as_millis());
}

pub fn chapter10_challenge() {
    println!("Chapter 10 challenge with multi-threading ...");
    let now = Instant::now();

    let light = PointLight::new(Tuple::Point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    // Floor
    let floor = Plane::new(
        Matrix::Identity(),
        Material::Phong(Phong {
            pattern: Pattern::Striped(Striped::new(
                Color::BLACK(),
                Color::WHITE(),
                Matrix::Identity(),
            )),
            specular: 0.0,
            ..Default::default()
        }),
    );

    let too_left_sphere = Sphere::new(
        Matrix::Translation(-6.0, 3.0, 10.0),
        Material::Phong(Phong {
            pattern: Pattern::Ring(Ring::new(
                Color::RED(),
                Color::WHITE(),
                Matrix::rotation_Y(-PI_BY_6) * Matrix::Scaling(0.2, 0.2, 0.2),
            )),
            ..Default::default()
        }),
    );

    let left_sphere = Sphere::new(
        Matrix::Translation(-1.5, 0.33, -0.75) * Matrix::Scaling(0.33, 0.33, 0.33),
        Material::Phong(Phong {
            pattern: Pattern::Striped(Striped::new(
                Color::RED(),
                Color::WHITE(),
                Matrix::rotation_Z(-PI_BY_6)
                    * Matrix::Translation(-1.0, 0.0, 0.0)
                    * Matrix::Scaling(0.2, 1.0, 1.0),
            )),
            ..Default::default()
        }),
    );

    let mid_sphere = Sphere::new(
        Matrix::Translation(-0.5, 1.0, 1.5),
        Material::Phong(Phong {
            pattern: Pattern::Gradient(Gradient::new(
                Color::RED(),
                Color::GREEN(),
                Matrix::rotation_Z(PI_BY_2)
                    * Matrix::Translation(-1.0, 0.0, 0.0)
                    * Matrix::Scaling(2.0, 1.0, 1.0),
            )),
            diffuse: 0.9,
            specular: 1.8,
            ..Default::default()
        }),
    );

    let right_sphere = Sphere::new(
        Matrix::Translation(1.5, 0.5, -0.5) * Matrix::Scaling(0.5, 0.5, 0.5),
        Material::Phong(Phong {
            pattern: Pattern::Checkers(Checkers::new(
                Color::BLUE(),
                Color::WHITE(),
                Matrix::rotation_Z(PI_BY_6) * Matrix::Scaling(0.4, 0.4, 0.4),
                true,
            )),
            ..Default::default()
        }),
    );

    let world = World::new(
        vec![light],
        vec![
            Body::from(floor),
            Body::from(too_left_sphere),
            Body::from(left_sphere),
            Body::from(mid_sphere),
            Body::from(right_sphere),
        ],
        vec![],
        0,
    );

    let camera = Camera::new(1620, 1080, PI_BY_3).look_at_from_position(
        Tuple::Point(0.0, 1.5, -5.0),
        Tuple::Point(0.0, 1.0, 0.0),
        Tuple::Vector(0.0, 1.0, 0.0),
    );

    camera
        .render_par(&world)
        .save_as_ppm("challenges/ch10.ppm")
        .unwrap();
    let elapsed = now.elapsed();
    println!("time taken: {} ms", elapsed.as_millis());
}

pub fn chapter11_challenge() {
    println!("Chapter 11 challenge with multi-threading ...");
    let now = Instant::now();

    let light = PointLight::new(Tuple::Point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    // Floor
    let floor = Plane::new(
        Matrix::Identity(),
        Material::Phong(Phong {
            pattern: Pattern::Checkers(Checkers::new(
                Color::BLACK(),
                Color::WHITE(),
                Matrix::Identity(),
                false,
            )),
            specular: 0.0,
            reflectiveness: 0.5,
            ..Default::default()
        }),
    );

    let left_sphere = Sphere::new(
        Matrix::Translation(-1.5, 0.33, -0.75) * Matrix::Scaling(0.33, 0.33, 0.33),
        Material::Phong(Phong {
            pattern: Pattern::Gradient(Gradient::new(
                Color::RED(),
                Color::GREEN(),
                Matrix::rotation_Z(270f64 / 180f64 * PI)
                    * Matrix::Translation(1.0, 0.0, 0.0)
                    * Matrix::Scaling(2.0, 2.0, 2.0),
            )),
            ..Default::default()
        }),
    );

    let mid_sphere = Sphere::new(
        Matrix::Translation(-0.5, 1.0, 0.5),
        Material::Phong(Phong {
            pattern: Pattern::Flat(Flat::new(Color::BLACK())),
            diffuse: 0.1,
            specular: 0.3,
            shininess: 200.0,
            // reflectiveness: 1.0,
            transparency: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        }),
    );

    let right_sphere = Sphere::new(
        Matrix::Translation(1.5, 0.5, -0.5) * Matrix::Scaling(0.5, 0.5, 0.5),
        Material::Phong(Phong {
            pattern: Pattern::Flat(Flat::new(Color::BLACK())),
            shininess: 1000.0,
            refractive_index: 1.5,
            // transparency: 1.0,
            reflectiveness: 1.0,
            ..Default::default()
        }),
    );

    let world = World::new(
        vec![light],
        vec![
            Body::from(floor),
            Body::from(left_sphere),
            Body::from(mid_sphere),
            Body::from(right_sphere),
        ],
        vec![],
        5,
    );

    let camera = Camera::new(1620, 1080, PI_BY_3).look_at_from_position(
        Tuple::Point(0.0, 1.5, -5.0),
        Tuple::Point(0.0, 1.0, 0.0),
        Tuple::Vector(0.0, 1.0, 0.0),
    );

    camera
        .render_par(&world)
        .save_as_ppm("challenges/ch11-refract.ppm")
        .unwrap();

    let elapsed = now.elapsed();
    println!("time taken: {} ms", elapsed.as_millis());
}

pub fn chapter12_challenge() {
    println!("Chapter 12 challenge with multi-threading ...");
    let now = Instant::now();

    let light = PointLight::new(Tuple::Point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    // Floor
    let floor = Plane::new(
        Matrix::Identity(),
        Material::Phong(Phong {
            pattern: Pattern::Checkers(Checkers::new(
                Color::BLACK(),
                Color::WHITE(),
                Matrix::Identity(),
                false,
            )),
            specular: 0.0,
            reflectiveness: 0.5,
            ..Default::default()
        }),
    );

    let cube = Cube::new(
        Matrix::Translation(0, 2, 0) * Matrix::rotation_Y(0.955531) * Matrix::rotation_X(-PI_BY_6),
        Material::Phong(Phong {
            reflectiveness: 0.2,
            transparency: 0.8,
            refractive_index: 1.5,
            ..Default::default()
        }),
    );

    let world = World::new(
        vec![light],
        vec![Body::from(floor), Body::from(cube)],
        vec![],
        5,
    );

    let camera = Camera::new(1620, 1080, PI_BY_3).look_at_from_position(
        Tuple::Point(0.0, 3.5, -15.0),
        Tuple::Point(0.0, 1.0, 0.0),
        Tuple::Vector(0.0, 1.0, 0.0),
    );

    camera
        .render_par(&world)
        .save_as_ppm("challenges/ch12.ppm")
        .unwrap();

    let elapsed = now.elapsed();
    println!("time taken: {} ms", elapsed.as_millis());
}

pub fn chapter13_challenge() {
    println!("Chapter 13 challenge with multi-threading ...");
    let now = Instant::now();

    let light = PointLight::new(Tuple::Point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    // Floor
    let floor = Plane::new(
        Matrix::Identity(),
        Material::Phong(Phong {
            pattern: Pattern::Checkers(Checkers::new(
                Color::BLACK(),
                Color::WHITE(),
                Matrix::Identity(),
                false,
            )),
            // specular: 0.0,
            reflectiveness: 0.5,
            ..Default::default()
        }),
    );

    let cyl = Cylinder::new(
        Matrix::Translation(-5, 2, 0) * Matrix::rotation_X(-PI_BY_3) * Matrix::rotation_Z(PI_BY_6),
        Material::Phong(Phong {
            pattern: Pattern::Gradient(Gradient::new(
                Color::RED(),
                Color::GREEN(),
                Matrix::Translation(-1, 0, 0) * Matrix::Scaling(2, 1, 1),
            )),
            reflectiveness: 0.1,
            // transparency: 0.5,
            // refractive_index: 1.5,
            ..Default::default()
        }),
        2.0,
        true,
    );

    let dcone = DoubleCone::new(
        Matrix::Translation(5, 2, 0), //* Matrix::rotation_X(PI_BY_3),// * Matrix::rotation_Z(PI_BY_6),
        Material::Phong(Phong {
            reflectiveness: 0.1,
            pattern: Pattern::Striped(Striped::new(
                Color::BLUE(),
                Color::WHITE(),
                Matrix::rotation_Z(-PI_BY_2) * Matrix::Scaling(0.1, 0.1, 0.1),
            )),
            // transparency: 0.5,
            // refractive_index: 1.5,
            ..Default::default()
        }),
        2.0,
        true,
    );

    let world = World::new(
        vec![light],
        vec![Body::from(floor), Body::from(cyl), Body::from(dcone)],
        vec![],
        5,
    );

    let camera = Camera::new(1620, 1080, PI_BY_3).look_at_from_position(
        Tuple::Point(0.0, 3.5, -15.0),
        Tuple::Point(0.0, 1.0, 0.0),
        Tuple::Vector(0.0, 1.0, 0.0),
    );

    camera
        .render_par(&world)
        .save_as_ppm("challenges/ch13.ppm")
        .unwrap();

    let elapsed = now.elapsed();
    println!("time taken: {} ms", elapsed.as_millis());
}

pub fn chapter14_challenge() {
    let now = Instant::now();

    let _floor = Plane::new(
        Matrix::Translation(0, 0, -10),
        Material::Phong(Phong {
            pattern: Pattern::Checkers(Checkers::new(
                Color::BLACK(),
                Color::WHITE(),
                Matrix::Identity(),
                false,
            )),
            specular: 0.0,
            reflectiveness: 0.5,
            ..Default::default()
        }),
    );

    let light = PointLight::new(Tuple::Point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let mut group = Group::new(Matrix::rotation_Z(-PI_BY_4), Vec::new());
    let s1 = Sphere::new(
        Matrix::Translation(2, 0, 0) * Matrix::Scaling(0.4, 0.4, 0.4),
        Material::Phong(Phong::default()),
    );
    let s2 = Cube::new(
        Matrix::Translation(0, 2, 0) * Matrix::Scaling(0.4, 0.4, 0.4),
        Material::Phong(Phong {
            pattern: Pattern::Flat(Flat::new(Color::RED())),
            ..Default::default()
        }),
    );
    let s3 = Sphere::new(
        Matrix::Translation(-2, 0, 0) * Matrix::Scaling(0.4, 0.4, 0.4),
        Material::Phong(Phong::default()),
    );
    let s4 = Sphere::new(
        Matrix::Translation(0, -2, 0) * Matrix::Scaling(0.4, 0.4, 0.4),
        Material::Phong(Phong::default()),
    );
    let cyl = Cylinder::new(
        Matrix::Translation(-5, 2, 0) * Matrix::rotation_X(-PI_BY_3) * Matrix::rotation_Z(PI_BY_6),
        Material::Phong(Phong {
            pattern: Pattern::Gradient(Gradient::new(
                Color::RED(),
                Color::GREEN(),
                Matrix::Translation(-1, 0, 0) * Matrix::Scaling(2, 1, 1),
            )),
            reflectiveness: 0.1,
            ..Default::default()
        }),
        2.0,
        true,
    );

    let dcone = DoubleCone::new(
        Matrix::Translation(5, 2, 0), //* Matrix::rotation_X(PI_BY_3),// * Matrix::rotation_Z(PI_BY_6),
        Material::Phong(Phong {
            reflectiveness: 0.1,
            pattern: Pattern::Striped(Striped::new(
                Color::BLUE(),
                Color::WHITE(),
                Matrix::rotation_Z(-PI_BY_2) * Matrix::Scaling(0.1, 0.1, 0.1),
            )),
            ..Default::default()
        }),
        2.0,
        true,
    );
    // Group::add_shape(s1.into(), &group);
    group.add_shape(s2.into());
    group.add_shape(s3.into());
    group.add_shape(s4.into());
    group.add_shape(cyl.into());
    group.add_shape(dcone.into());

    let world = World::new(
        vec![light],
        vec![
            // floor.into(),
            s1.into(),
            // s2.into(),
            // s3.into(),
            // s4.into(),
            // cyl.into(),
            // dcone.into(),
        ],
        vec![group.build()],
        5,
    );

    let camera = Camera::new(1620, 1080, PI_BY_3).look_at_from_position(
        Tuple::Point(0.0, 3.5, -15.0),
        Tuple::Point(0.0, 1.0, 0.0),
        Tuple::Vector(0.0, 1.0, 0.0),
    );

    camera
        .render_par(&world)
        .save_as_ppm("challenges/ch14.ppm")
        .unwrap();
    let elapsed = now.elapsed();
    println!("time taken: {} ms", elapsed.as_millis());
}
