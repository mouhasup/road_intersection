extern crate sdl2;
mod vehicules;
use rand::Rng;
use vehicules::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

const WAIT_DURATION: Duration = Duration::from_millis(300);
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut vehicles: Vec<Vehicule> = Vec::new();

    let mut directions: HashMap<&str, Direct> = HashMap::new();
    directions.insert(
        "up",
        Direct {
            name: "up".to_string(),
            color: Color::RED,
            state: false,
            rect_ligth: Rect::new(430, 430, 30, 30),
            stop: 430,
        },
    );
    directions.insert(
        "down",
        Direct {
            name: "down".to_string(),
            color: Color::RED,
            state: false,
            rect_ligth: Rect::new(340, 340, 30, 30),
            stop: 340,
        },
    );
    directions.insert(
        "left",
        Direct {
            name: "left".to_string(),
            color: Color::RED,
            state: false,
            rect_ligth: Rect::new(430, 340, 30, 30),
            stop: 430,
        },
    );
    directions.insert(
        "right",
        Direct {
            name: "right".to_string(),
            color: Color::RED,
            state: false,
            rect_ligth: Rect::new(340, 430, 30, 30),
            stop: 340,
        },
    );

    let mut last_event_time = Instant::now();

    'running: loop {
        let mut i = 0;
        while i < vehicles.len() {
            if vehicles[i].rect.x < 0
                || vehicles[i].rect.y > 800
                || vehicles[i].rect.x > 800
                || vehicles[i].rect.y < 0
            {
                vehicles.remove(i);
            } else {
                i += 1;
            }
        }

        let new_vehic: Vec<Vehicule> = vehicles.to_vec();
        let left_vehicles: Vec<Vehicule> = new_vehic
            .iter()
            .filter(|&v| {
                (v.direction == "left".to_string())
                    && v.is_befor_stop(directions.get("left").unwrap().stop)
            })
            .cloned()
            .collect();
        let right_vehicles: Vec<Vehicule> = new_vehic
            .iter()
            .filter(|&v| {
                (v.direction == "right".to_string())
                    && v.is_befor_stop(directions.get("right").unwrap().stop)
            })
            .cloned()
            .collect();
        let up_vehicles: Vec<Vehicule> = new_vehic
            .iter()
            .filter(|&v| {
                (v.direction == "up".to_string())
                    && v.is_befor_stop(directions.get("up").unwrap().stop)
            })
            .cloned()
            .collect();

        let down_vehicles: Vec<Vehicule> = new_vehic
            .iter()
            .filter(|&v| {
                (v.direction == "down".to_string())
                    && v.is_befor_stop(directions.get("down").unwrap().stop)
            })
            .cloned()
            .collect();

        // Calculez la longueur des vecteurs avant le match
        let left_len = left_vehicles.len();
        let right_len = right_vehicles.len();
        let up_len = up_vehicles.len();
        let down_len = down_vehicles.len();

        if !some_on_in_intersect(vehicles.clone()) {
            // Utilisez les valeurs calculées dans le match
            match left_len.max(right_len).max(up_len).max(down_len) {
                len if len == left_len => {
                    for (k, v) in &mut directions {
                        if k == &"left" {
                            v.state = true;
                            v.color = Color::GREEN;
                        } else {
                            v.state = false;
                            v.color = Color::RED;
                        }
                    }
                }
                len if len == right_len => {
                    for (k, v) in &mut directions {
                        if k == &"right" {
                            v.state = true;
                            v.color = Color::GREEN;
                        } else {
                            v.state = false;
                            v.color = Color::RED;
                        }
                    }
                }
                len if len == up_len => {
                    for (k, v) in &mut directions {
                        if k == &"up" {
                            v.state = true;
                            v.color = Color::GREEN;
                        } else {
                            v.state = false;
                            v.color = Color::RED;
                        }
                    }
                }
                len if len == down_len => {
                    for (k, v) in &mut directions {
                        if k == &"down" {
                            v.state = true;
                            v.color = Color::GREEN;
                        } else {
                            v.state = false;
                            v.color = Color::RED;
                        }
                    }
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for event in event_pump.poll_iter() {
            if Instant::now().duration_since(last_event_time) >= WAIT_DURATION {
                // Réinitialiser le temps du dernier événement
                last_event_time = Instant::now();
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        ..
                    } => {
                        if vehicles.len() < 16 {
                            vehicles.push(Vehicule::new("up"));
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        ..
                    } => {
                        if vehicles.len() < 16 {
                            vehicles.push(Vehicule::new("down"));
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => {
                        if vehicles.len() < 16 {
                            vehicles.push(Vehicule::new("left"));
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        if vehicles.len() < 16 {
                            vehicles.push(Vehicule::new("right"));
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::R),
                        ..
                    } => {
                        let direction = ["up", "down", "left", "right"];
                        let index = rand::thread_rng().gen_range(0, 4);
                        if vehicles.len() < 16 {
                            vehicles.push(Vehicule::new(direction[index]));
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::X),
                        ..
                    } => {
                        vehicles.clear();
                    }
                    _ => {}
                }
            }
        }

        canvas.set_draw_color(Color::WHITE);
        let _ = canvas.draw_line(Point::new(370, 0), Point::new(370, 370));
        let _ = canvas.draw_line(Point::new(0, 400), Point::new(370, 400));
        let _ = canvas.draw_line(Point::new(430, 0), Point::new(430, 370));

        let _ = canvas.draw_line(Point::new(0, 370), Point::new(370, 370));
        let _ = canvas.draw_line(Point::new(400, 0), Point::new(400, 370));
        let _ = canvas.draw_line(Point::new(0, 430), Point::new(370, 430));

        let _ = canvas.draw_line(Point::new(430, 370), Point::new(800, 370));
        let _ = canvas.draw_line(Point::new(430, 400), Point::new(800, 400));
        let _ = canvas.draw_line(Point::new(430, 430), Point::new(800, 430));

        let _ = canvas.draw_line(Point::new(370, 430), Point::new(370, 800));
        let _ = canvas.draw_line(Point::new(400, 430), Point::new(400, 800));
        let _ = canvas.draw_line(Point::new(430, 430), Point::new(430, 800));

        canvas.set_draw_color(Color::RED);
        let _ = canvas.draw_rect(Rect::new(340, 340, 30, 30));

        // Draw vehicules
        for direction in directions.values() {
            canvas.set_draw_color(direction.color);
            let _ = canvas.draw_rect(direction.rect_ligth);
        }
        let mut all_vehic: Vec<Vehicule> = vehicles.to_vec();

        // Move vehicles:
        for vehicule in vehicles.iter_mut() {
            for (k, v) in &mut directions {
                if k == &vehicule.direction {
                    vehicule.moves(v.stop, v.state, &mut all_vehic);
                    // print!("{}",k);
                    break;
                }
            }
            canvas.set_draw_color(vehicule.color);
            let _ = canvas.fill_rect(vehicule.rect);
        }

        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn some_on_in_intersect(vehicles: Vec<Vehicule>) -> bool {
    for v in vehicles {
        if (v.rect.x < 430 && v.rect.x > 340) && (v.rect.y < 430 && v.rect.y > 340) {
            return true;
        }
    }
    return false;
}
