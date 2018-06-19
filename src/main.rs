extern crate piston_window;

use piston_window::*;

use std::collections::HashSet;

struct Entity {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    width: f64,
    height: f64,
    color: [f32; 4],
}

impl Entity {
    fn draw(&self, c: Context, g: &mut G2d) {
        rectangle(
            self.color,
            [self.x, self.y, self.width, self.height],
            c.transform, g
        );
    }

    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    fn step_back(&mut self) {
        self.x -= self.dx;
        self.y -= self.dy;
    }

    fn intersects(&self, other: &Entity) -> bool {
        !(
            self.x > other.x + other.width ||
            self.x + self.width < other.x ||
            self.y > other.y + other.height ||
            self.y + self.height < other.y
        )
    }

    fn contains(&self, other: &Entity) -> bool {
        other.x >= self.x &&
        other.x + other.width <= self.x + self.width &&
        other.y >= self.y &&
        other.y + other.height <= self.y + self.height
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings
        ::new("Pong", (640, 480))
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .vsync(true)
        .build()
        .expect("Unable to instantiate new window");

    let mut keys_down: HashSet<Key> = HashSet::new();

    let mut player = Entity {
        x: 10.,
        y: 50.,
        dx: 0.,
        dy: 0.,
        width: 10.,
        height: 60.,
        color: [1.0, 0.0, 0.0, 1.0],
    };

    let mut enemy = Entity {
        x: 400.,
        y: 50.,
        dx: 0.,
        dy: 0.,
        width: 10.,
        height: 60.,
        color: [0.0, 0.0, 1.0, 1.0],
    };

    let mut ball = Entity {
        x: 200.,
        y: 100.,
        dx: 1.,
        dy: 0.7,
        width: 8.,
        height: 8.,
        color: [0.0, 0.0, 0.0, 1.0],
    };

    let field = Entity {
        x: 0.,
        y: 0.,
        dx: 0.,
        dy: 0.,
        width: 450.,
        height: 200.,
        color: [1., 1., 1., 1.],
    };

    let goal_enemy = Entity {
        x: 445.,
        y: 0.,
        dx: 0.,
        dy: 0.,
        width: 5.,
        height: 200.,
        color: [0.5, 0.5, 0.5, 1.],
    };

    let goal_player = Entity {
        x: 0.,
        y: 0.,
        dx: 0.,
        dy: 0.,
        width: 5.,
        height: 200.,
        color: [0.5, 0.5, 0.5, 1.],
    };

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            keys_down.insert(key);
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            keys_down.remove(&key);
        }

        if event.update_args().is_some() {
            // Move player
            player.dy = 0.;
            if keys_down.contains(&Key::Down) {
                player.dy = 1.;
            }
            else if keys_down.contains(&Key::Up) {
                player.dy = -1.;
            }

            player.step();
            if !field.contains(&player) {
                player.step_back();
            }

            // Set enemy moving direction based on ball position
            enemy.dy = 0.;
            if ball.y < enemy.y + (enemy.height / 2.) {
                enemy.dy = -1.;
            }
            else if ball.y > enemy.y + (enemy.height / 2.) {
                enemy.dy = 1.;
            }

            // Move enemy while keeping contained in field
            enemy.step();
            if !field.contains(&enemy) {
                enemy.step_back();
            }

            // Move ball
            ball.step();

            // Bounce ball from paddles and increase speed
            if ball.intersects(&enemy) || ball.intersects(&player) {
                ball.step_back();
                ball.dx *= -1.2;
                ball.dy *= 1.2;
            }

            // Reset ball to center field when a goal is hit
            if ball.intersects(&goal_player) || ball.intersects(&goal_enemy) {
                ball.x = (field.width / 2.) - (ball.width / 2.);
                ball.y = (field.height / 2.) - (ball.height / 2.);
                ball.dx = 1.;
                ball.dy = 0.7;
            }

            // Bounce ball from bottom/top of field
            if !field.contains(&ball) {
                ball.step_back();
                ball.dy *= -1.;
            }
        }

        if event.render_args().is_some() {
            window.draw_2d(&event, |c, g| {
                clear([0., 0., 0., 1.], g);
                field.draw(c, g);
                goal_player.draw(c, g);
                goal_enemy.draw(c, g);
                player.draw(c, g);
                enemy.draw(c, g);
                ball.draw(c, g);
            });
        }
    }
}
