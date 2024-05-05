use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Debug, Clone)]
pub struct Vehicule {
    pub color: Color,
    pub direction: String,
    // pub direc: Direct,
    pub speed: i32,
    pub rect: Rect,
    pub turn: String,
    pub turned: bool,
}

impl Vehicule {
    pub fn new(direct: &str) -> Self {
        let rend_color: Color = randow_color();
        Self {
            color: rend_color,
            direction: direct.to_string(),
            speed: 2,
            rect: {
                match direct {
                    "up" => Rect::new(400, 770, 30, 30),
                    "down" => Rect::new(370, 0, 30, 30),
                    "left" => Rect::new(770, 370, 30, 30),
                    "right" => Rect::new(0, 400, 30, 30),
                    _ => todo!(),
                }
            },
            turn: {
                match rend_color {
                    Color::YELLOW => "right".to_string(),
                    Color::RED => "left".to_string(),
                    _ => "forword".to_string(),
                }
            },
            turned: false,
        }
    }
    pub fn add_step(&mut self) {
        println!("x:{} y:{}", self.rect.x, self.rect.y);
        match self.direction.as_str() {
            "up" => self.rect.y -= self.speed,
            "down" => self.rect.y += self.speed,
            "left" => self.rect.x -= self.speed,
            "right" => self.rect.x += self.speed,
            _ => {}
        }
    }

    pub fn moves(&mut self, stop: i32, is_green: bool, vehicles: &mut Vec<Vehicule>) {
        if !self.turned && self.update(stop) {
            match (self.direction.as_str(), self.turn.as_str()) {
                ("up", "right") => self.direction = "right".to_string(),
                ("left", "right") => self.direction = "up".to_string(),
                ("down", "right") => self.direction = "left".to_string(),
                ("right", "right") => self.direction = "down".to_string(),

                ("up", "left") => self.direction = "left".to_string(),
                ("left", "left") => self.direction = "down".to_string(),
                ("down", "left") => self.direction = "right".to_string(),
                ("right", "left") => self.direction = "up".to_string(),
                _ => todo!(),
            }
            self.turned = true;
        }

        if self.is_befor_stop(stop) {
            if self.is_at_stop(stop, vehicles) {
                if is_green {
                    self.add_step();
                    println!("{}", "is at stop and green");
                }
                println!("{}", "is at stop");
            } else {
                // println!("{}{}", "is at stop", stop);
                println!("{}{}", "not at stop", stop);
                self.add_step();
            }
        } else {
            println!("{}{}", "after stop", self.direction);
            self.add_step();
        }
    }

    pub fn is_at_stop(&mut self, stop: i32, vehicles: &mut Vec<Vehicule>) -> bool {
        if self.direction == "up".to_string() {
            let mut vec_same_direct = avoid_collision(vehicles, "up".to_string(), stop);
            vec_same_direct.sort_by_key(|v| v.rect.y);
            println!("{} len", vec_same_direct.len());
            for i in 0..vec_same_direct.len() {
                if vec_same_direct[i].rect.y == self.rect.y {
                    if i == 0 {
                        println!("{} stop ", i);
                        return (self.rect.y - self.speed) == stop;
                    }
                    println!("{} collis", i);
                    return (self.rect.y - self.speed) < vec_same_direct[i - 1].rect.y + 50;
                }
            }
        } else if self.direction == "down".to_string() {
            let mut vec_same_direct = avoid_collision(vehicles, "down".to_string(), stop);

            vec_same_direct.sort_by_key(|v| v.rect.y);
            for i in 0..vec_same_direct.len() {
                if vec_same_direct[i].rect.y == self.rect.y {
                    if i == vec_same_direct.len() - 1 {
                        return (self.rect.y + self.speed) > stop;
                    }
                    return (self.rect.y + self.speed) > vec_same_direct[i + 1].rect.y - 50;
                }
            }
        } else if self.direction == "left".to_string() {
            let mut vec_same_direct = avoid_collision(vehicles, "left".to_string(), stop);
            vec_same_direct.sort_by_key(|v| v.rect.x);
            for i in 0..vec_same_direct.len() {
                if vec_same_direct[i].rect.x == self.rect.x {
                    if i == 0 {
                        return (self.rect.x - self.speed) < stop;
                    }
                    return (self.rect.x - self.speed) < vec_same_direct[i - 1].rect.x + 50;
                }
            }
        } else if self.direction == "right".to_string() {
            let mut vec_same_direct = avoid_collision(vehicles, "right".to_string(), stop);
            vec_same_direct.sort_by_key(|v| v.rect.x);
            for i in 0..vec_same_direct.len() {
                if vec_same_direct[i].rect.x == self.rect.x {
                    if i == vec_same_direct.len() - 1 {
                        return (self.rect.x + self.speed) > stop;
                    }

                    return (self.rect.x + self.speed) > vec_same_direct[i + 1].rect.x - 50;
                }
            }
        }
        // match self.direction.as_str(){
        //     "up" => (self.rect.y + self.speed) < stop,
        //     "down" => (self.rect.y + self.speed) > stop,
        //     "left" => (self.rect.x + self.speed) < stop,
        //     "right" => (self.rect.x + self.speed) > stop,
        //     _ => todo!()
        // }
        true
    }

    pub fn is_befor_stop(&self, stop: i32) -> bool {
        match self.direction.as_str() {
            "up" => (self.rect.y) >= stop,
            "down" => self.rect.y <= stop,
            "left" => self.rect.x >= stop,
            "right" => self.rect.x <= stop,
            _ => false,
        }
    }

    pub fn update(&self, stop: i32) -> bool {
        match self.turn.as_str() {
            "left" => match self.direction.as_str() {
                "up" => self.rect.y == stop - 60,
                "down" => self.rect.y == stop + 60,
                "left" => self.rect.x == stop - 60,
                "right" => self.rect.x == stop + 60,
                _ => todo!(),
            },
            "right" => match self.direction.as_str() {
                "up" => self.rect.y == stop - 30,
                "down" => self.rect.y == stop + 30,
                "left" => self.rect.x == stop - 30,
                "right" => self.rect.x == stop + 30,
                _ => todo!(),
            },
            _ => false,
        }
    }
}

pub struct Direct {
    pub name: String,
    pub color: Color,
    pub state: bool,
    pub rect_ligth: Rect,
    pub stop: i32,
    // pub direction: String,
}

pub fn randow_color() -> Color {
    let color = vec![Color::YELLOW, Color::BLUE, Color::RED];
    let rng = rand::thread_rng().gen_range(0, 3);
    color[rng]
}

pub fn avoid_collision(
    vehicules: &mut Vec<Vehicule>,
    directin: String,
    stop: i32,
) -> Vec<&Vehicule> {
    vehicules
        .iter()
        .filter(|&v| (v.direction == directin) && v.is_befor_stop(stop))
        .collect()
}
// pub struct Light {
//     pub color: Color,
//     pub state: bool,
//     pub direction: String,
// }

// impl Light{
//     pub fn new(col: Color, state: bool, direct: String) -> Self{
//         Self { color: col, state: state, direction: direct }
//     }
// }
