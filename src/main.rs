use tetra::graphics::{self, Color, Texture, Rectangle};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;


const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;

struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
}

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position,
            velocity,
        }
    }

    fn hits_top(&self) -> bool {
        return self.position.y <= 4.0;
    }

    fn hits_bottom(&self) -> bool {
        return (self.position.y + self.height()) >= (WINDOW_HEIGHT - 4.0);
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(self.position.x, self.position.y, self.width(), self.height())
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./resources/paddle1.png")?;
        let player1_position = Vec2::new(16.0, (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0);

        let player2_texture = Texture::new(ctx, "./resources/paddle2.png")?;
        let player2_position = Vec2::new(WINDOW_WIDTH - player2_texture.width() as f32 - 16.0, (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0);

        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
        );
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);


        let player1 = Entity::new(player1_texture, player1_position);
        let player2 = Entity::new(player2_texture, player2_position);
        let ball = Entity::with_velocity(ball_texture, ball_position, ball_velocity);
        Ok(GameState { player1, player2, ball })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) && !self.player1.hits_top() {
            self.player1.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::S) && !self.player1.hits_bottom() {
            self.player1.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up) && !self.player2.hits_top() {
            self.player2.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::Down) && !self.player2.hits_bottom() {
            self.player2.position.y += PADDLE_SPEED;
        }

        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2)
        } else {
            None
        };

        if paddle_hit.is_some() {
            self.ball.velocity.x = -self.ball.velocity.x;
        }
        self.ball.position += self.ball.velocity;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        graphics::draw(ctx, &self.player1.texture, self.player1.position);
        graphics::draw(ctx, &self.player2.texture, self.player2.position);
        graphics::draw(ctx, &self.ball.texture, self.ball.position);

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
