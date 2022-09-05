use minifb::{Key, KeyRepeat, Window, WindowOptions};
use renderer::{Line, Renderer, Square};
use utils::{Color, Position, HEIGHT, PI, WIDTH};

mod renderer;
mod utils;

fn main() {
    let mut renderer: Renderer = Renderer {
        buffer: vec![0; WIDTH * HEIGHT],
    };

    let mut window = Window::new(
        "Meshworks 3D Graphics Engine",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut player = Player {
        fPlayerX: 4.0,
        fPlayerY: 4.0,
        fPlayerA: 0.0,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();
        (&mut renderer, &window);

        renderer.clear(Color::BLACK);
        player.move_angle(&window);
        player.move_player(&window);
        player.game(&mut renderer);
    }
}

struct Player {
    fPlayerX: f32,
    fPlayerY: f32,
    fPlayerA: f32,
}
impl Player {
    fn move_angle(&mut self, window: &Window) {
        if (self.fPlayerA > PI * 2.0) {
            self.fPlayerA = 0.0;
        }
        if (self.fPlayerA < 0.0) {
            self.fPlayerA = PI * 2.0;
        }
        if (window.is_key_down(Key::A)) {
            self.fPlayerA -= 0.1;
        }
        if (window.is_key_down(Key::D)) {
            self.fPlayerA += 0.1;
        }
        println!("{}", self.fPlayerA);
    }

    fn move_player(&mut self, window: &Window) {
        let speed = 0.4;

        if (window.is_key_down(Key::W)) {
            self.fPlayerX += self.fPlayerA.sin() * speed;
            self.fPlayerY += self.fPlayerA.cos() * speed;
        }
        if (window.is_key_down(Key::S)) {
            self.fPlayerX += self.fPlayerA.sin() * speed;
            self.fPlayerY += self.fPlayerA.cos() * speed;
        }
    }

    fn game(&mut self, renderer: &mut Renderer) {
        let nMapHeight = 16;
        let nMapWidth = 16;

        let fFOV = PI / 4.0;
        let fDepth = 16.0;

        let mapString =
            String::from("1111111111111111100000000000000110000000000000011000000000000001100000000000000110000000000000011000000000000001100000000000000110000000000000011000000000000001100000000000000110000000000000011000000000000001100000000000000110000000000000011111111111111111");

        let mapCode: Vec<_> = mapString.chars().collect();

        for x in 0..WIDTH {
            // for hver column, udregn den projiterede vinkel til world space
            let mut fRayAngle = (self.fPlayerA - fFOV / 2.0) + (x as f32 / WIDTH as f32) * fFOV;

            let mut fDistanceToWall = 0.0;
            let mut bHitWall = false;

            let mut fEyeX = fRayAngle.sin();
            let mut fEyeY = fRayAngle.cos();

            while bHitWall == false && fDistanceToWall < fDepth {
                fDistanceToWall += 0.1;

                let mut fRayAngle = (self.fPlayerA - fFOV / 2.0) + (x as f32 / WIDTH as f32) * fFOV;
                let mut nTestX = (self.fPlayerX + fEyeX * fDistanceToWall) as i32;
                let mut nTestY = (self.fPlayerY + fEyeY * fDistanceToWall) as i32;

                // test om ray er out of bounds
                if nTestX < 0 || nTestX >= nMapWidth || nTestY < 0 || nTestY >= nMapHeight {
                    fDistanceToWall = fDepth;
                    continue; // bare sæt distance til max depth
                } else {
                    {
                        if mapCode[((nTestY - 1) * nMapWidth + nTestX) as usize] == '1' {
                            bHitWall = true;
                        }
                    }
                }

                // calculate distance to ceiling and floor
                let mut nCeiling = (HEIGHT as f32 / 2.0) - (HEIGHT as f32 / fDistanceToWall);
                let mut nFloor = HEIGHT as f32 - nCeiling;

                for y in 0..HEIGHT {
                    if y < nCeiling as usize {
                        &renderer.draw_pixel(
                            Position {
                                x: x as u32,
                                y: y as u32,
                            },
                            Color::BLACK,
                        );
                    } else if y > nCeiling as usize && y <= nFloor as usize {
                        &renderer.draw_pixel(
                            Position {
                                x: x as u32,
                                y: y as u32,
                            },
                            Color::WHITE,
                        );
                    } else {
                        &renderer.draw_pixel(
                            Position {
                                x: x as u32,
                                y: y as u32,
                            },
                            Color::DARK_GREY,
                        );
                    }
                }
            }
        }
    }
}
