use std::io::{self, Write};
use std::ops::{Add, Div, Sub, Mul};

struct Ray {
    _origin: Vec3,
    dir: Vec3,
}

impl Ray {
    fn with_values(_origin: Vec3, dir: Vec3) -> Ray {
        Ray {_origin, dir}
    }

    fn direction(&self) -> &Vec3 {
        &self.dir
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z
        }
    }
}

#[derive(Copy, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn with_values(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x, y, z}
    }

    fn x(self) -> f64 {
        self.x
    }

    fn y(self) -> f64 {
        self.y
    }

    fn z(self) -> f64 {
        self.z
    }

    fn length(self) -> f64 {
        let dot = self.x * self.x + self.y * self.y + self.z * self.z;
        dot.sqrt()
    }

    fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }
}

type Color = Vec3;

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

//color.rs
//fn write_color(out: &mut io::BufWriter<io::Stdout>, pixel_color: &Color) {
fn write_color<W: Write>(out: &mut W, pixel_color: &Color) {
    let range_factor: f64 = 255.999;

    let rbyte = (range_factor * pixel_color.x()) as i32;
    let gbyte = (range_factor * pixel_color.y()) as i32;
    let bbyte = (range_factor * pixel_color.z()) as i32;

    write!(out, "{} {} {}\n", rbyte, gbyte, bbyte).unwrap();
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = Vec3::unit_vector(&ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::with_values(1.0, 1.0, 1.0) + a * Color::with_values(0.5, 0.7, 1.0)
}


fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = Vec3::with_values(0.0, 0.0, 0.0);

    let viewport_u = Vec3::with_values(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::with_values(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upperleft = camera_center
        - Vec3::with_values(0.0, 0.0, focal_length)
        - (viewport_u / 2.0)
        - (viewport_v / 2.0);

    let pixel00_loc = viewport_upperleft + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3\n{} {}\n255", image_width, image_height);

    let mut writer = io::BufWriter::new(io::stdout().lock());

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);

        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::with_values(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&mut writer, &pixel_color);
        }
    }
}
