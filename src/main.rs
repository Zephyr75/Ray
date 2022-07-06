extern crate sdl2; 

use sdl2::{pixels::Color, video::Window};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

use std::time::{SystemTime, Duration};
use rayon::prelude::*;

mod vector3;
use vector3::Vector3;
mod point3;
use point3::Point3;
mod ray;
use ray::Ray;

const WIDTH: u32 = 1200;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const HEIGHT: u32 = 675;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("Raytracer", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_frame = SystemTime::now() - Duration::from_millis(1);

    // Viewport settings
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO as f32 * viewport_height;
    let focal_length = 1.0;

    let origin: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let horizontal: Vector3 = Vector3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };

    let vertical: Vector3 = Vector3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };

    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vector3 :: Vector3{x: 0.0, y: 0.0, z: focal_length};




    'running: loop {
        //compute fps
        println!("fps: {}", 1000 / SystemTime::now().duration_since(last_frame).unwrap().as_millis());
        last_frame = SystemTime::now();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        ////////////////////////////////////////////////////////////////////////

        let indexes = 0 .. HEIGHT * WIDTH;

        //array of (Point, Color) of size WIDTH * HEIGHT
        let mut pixels: Vec<(Point, Color)> = vec![];

        indexes.into_par_iter().for_each(|x| {
            let i = x % WIDTH;
            let j = x / WIDTH;
            let u: f32 = i as f32 / (WIDTH as f32 - 1.0);
            let v: f32 = j as f32 / (HEIGHT as f32 - 1.0);
            
            let r: Ray = Ray { origin: origin, direction: lower_left_corner - origin + horizontal * u + vertical * v };

            let mut color = ray_color(r);
            
            let p: Point = Point::new(i as i32, (HEIGHT - j - 1) as i32);

            pixels.push((p, color));
        });

        pixels.iter().for_each(|p|{
            canvas.set_draw_color(p.1);
            canvas.draw_point(p.0);
        });

        ////////////////////////////////////////////////////////////////////////

        canvas.present();
    }
}

fn ray_color(r: Ray) -> Color {
    let center: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let mut t = hit_sphere(center, 0.5, r);
    if (t > 0.0){
        let N = (r.at(t) - center).unit();
        let r = 255.0 * 0.5 * (N.x + 1.0);
        let g = 255.0 * 0.5 * (N.y + 1.0);
        let b = 255.0 * 0.5 * (N.z + 1.0);
        return Color::RGB(r as u8, g as u8, b as u8);
    }

    let unit_direction: Vector3 = r.direction.unit();
    t = 0.5*(unit_direction.y + 1.0);
    let r = (1.0 - t) * 255.0 + t * 127.0;
    let g = (1.0 - t) * 255.0 + t * 180.0;
    let b = (1.0 - t) * 255.0 + t * 255.0;
    Color::RGB(r as u8, g as u8, b as u8)
}

fn hit_sphere(center: Point3, radius: f32, r: Ray) -> f32 {
    let oc: Vector3 = r.origin - center;
    let a: f32 = oc.dot(oc) - radius * radius;
    let b: f32 = 2.0 * oc.dot(r.direction);
    let c: f32 = r.direction.dot(r.direction);
    let discriminant: f32 = b * b - 4.0 * c * a;
    if (discriminant < 0.0) {
        -1.0
    } else {
        (-b - discriminant.sqrt() ) / (2.0*c)
    }
}