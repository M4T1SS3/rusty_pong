use ggez::{graphics, Context, GameResult};
use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics::{Color, MeshBuilder, Rect, Text, Font};
use ggez::mint::Point2;
use ggez::mint::Vector2;
use ggez::input::keyboard;
use rand::Rng; 


// Constants for gameplay
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_SIZE: f32 = 10.0;
const PADDLE_SPEED: f32 = 425.0;
const BALL_SPEED: f32 = 300.0;


struct GameState {
    player1_pos: f32,
    player2_pos: f32,
    ball_pos: Point2<f32>,
    ball_vel: Vector2<f32>,
    player1_score: i32,
    player2_score: i32,
}
impl GameState {
        pub fn new() -> GameResult<GameState> {
            let state = GameState {
                player1_pos: SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                player2_pos: SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                ball_pos: Point2 { x: SCREEN_WIDTH / 2.0 - BALL_SIZE / 2.0, y: SCREEN_HEIGHT / 2.0 - BALL_SIZE / 2.0 },
                ball_vel: Vector2 { x: -BALL_SPEED, y: 0.0 },
                player1_score: 0,
                player2_score: 0,
            };
            Ok(state)
        }

    fn update_score_and_reset(&mut self) {
        if self.ball_pos.x < 0.0 {
            self.player2_score += 1;  // Increment score for player 2
            self.reset_ball(true);
        } else if self.ball_pos.x > SCREEN_WIDTH {
            self.player1_score += 1;  // Increment score for player 1
            self.reset_ball(false);
        }
    }

    fn reset_ball(&mut self, to_right: bool) {
        let mut rng = rand::thread_rng();
        let angle: f32 = rng.gen_range(-1.0..1.0); // Generate a random angle

        self.ball_pos = Point2 { x: SCREEN_WIDTH / 2.0 - BALL_SIZE / 2.0, y: SCREEN_HEIGHT / 2.0 - BALL_SIZE / 2.0 };
        self.ball_vel.x = if to_right { BALL_SPEED } else { -BALL_SPEED };
        self.ball_vel.y = BALL_SPEED * angle; // Apply the random angle to the vertical velocity
    }
    
    fn check_paddle_collision(&mut self) {
        let paddle1_right_edge = 30.0 + PADDLE_WIDTH;
        let paddle2_left_edge = SCREEN_WIDTH - 40.0 - PADDLE_WIDTH;

        // Collision with left paddle
        if self.ball_pos.x <= paddle1_right_edge &&
           self.ball_pos.x + BALL_SIZE >= 30.0 && // Ensure ball front face hits the paddle
           self.ball_pos.y + BALL_SIZE >= self.player1_pos &&
           self.ball_pos.y <= self.player1_pos + PADDLE_HEIGHT &&
           self.ball_vel.x < 0.0 { // Ensure the ball is moving towards the paddle
            self.ball_vel.x = -self.ball_vel.x;
            let impact_offset = (self.ball_pos.y + BALL_SIZE / 2.0 - (self.player1_pos + PADDLE_HEIGHT / 2.0)) / PADDLE_HEIGHT;
            self.ball_vel.y += impact_offset * BALL_SPEED;
        }

        // Collision with right paddle
        if self.ball_pos.x + BALL_SIZE >= paddle2_left_edge &&
           self.ball_pos.x <= SCREEN_WIDTH - 40.0 && // Ensure ball front face hits the paddle
           self.ball_pos.y + BALL_SIZE >= self.player2_pos &&
           self.ball_pos.y <= self.player2_pos + PADDLE_HEIGHT &&
           self.ball_vel.x > 0.0 { // Ensure the ball is moving towards the paddle
            self.ball_vel.x = -self.ball_vel.x;
            let impact_offset = (self.ball_pos.y + BALL_SIZE / 2.0 - (self.player2_pos + PADDLE_HEIGHT / 2.0)) / PADDLE_HEIGHT;
            self.ball_vel.y += impact_offset * BALL_SPEED;
        }
    }
}

impl EventHandler for GameState {

    
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.check_paddle_collision();
        self.update_score_and_reset();
        
        // Player 1 Movement Constraints
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.player1_pos -= PADDLE_SPEED * ggez::timer::delta(ctx).as_secs_f32();
            if self.player1_pos < 0.0 {
                self.player1_pos = 0.0; // Prevent paddle from moving above the top edge
            }
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.player1_pos += PADDLE_SPEED * ggez::timer::delta(ctx).as_secs_f32();
            if self.player1_pos > SCREEN_HEIGHT - PADDLE_HEIGHT {
                self.player1_pos = SCREEN_HEIGHT - PADDLE_HEIGHT; // Prevent paddle from moving below the bottom edge
            }
        }

        // Player 2 Movement Constraints
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.player2_pos -= PADDLE_SPEED * ggez::timer::delta(ctx).as_secs_f32();
            if self.player2_pos < 0.0 {
                self.player2_pos = 0.0; // Same check for the second player
            }
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.player2_pos += PADDLE_SPEED * ggez::timer::delta(ctx).as_secs_f32();
            if self.player2_pos > SCREEN_HEIGHT - PADDLE_HEIGHT {
                self.player2_pos = SCREEN_HEIGHT - PADDLE_HEIGHT; // Same check for the second player
            }
        }

        // Ball movement and collision with walls
        self.ball_pos.x += self.ball_vel.x * ggez::timer::delta(ctx).as_secs_f32();
        self.ball_pos.y += self.ball_vel.y * ggez::timer::delta(ctx).as_secs_f32();
        
        if self.ball_pos.y <= 0.0 || self.ball_pos.y + BALL_SIZE >= SCREEN_HEIGHT {
            self.ball_vel.y = -self.ball_vel.y; // Reverse the ball's vertical direction if it hits the top or bottom wall
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        // Draw paddles and ball
        let paddle1 = MeshBuilder::new().rectangle(
            graphics::DrawMode::fill(),
            Rect::new(30.0, self.player1_pos, PADDLE_WIDTH, PADDLE_HEIGHT),
            Color::WHITE
        )?.build(ctx)?;
        
        let paddle2 = MeshBuilder::new().rectangle(
            graphics::DrawMode::fill(),
            Rect::new(SCREEN_WIDTH - 40.0, self.player2_pos, PADDLE_WIDTH, PADDLE_HEIGHT),
            Color::WHITE
        )?.build(ctx)?;

        let ball = MeshBuilder::new().rectangle(
            graphics::DrawMode::fill(),
            Rect::new(self.ball_pos.x, self.ball_pos.y, BALL_SIZE, BALL_SIZE),
            Color::WHITE
        )?.build(ctx)?;

        graphics::draw(ctx, &paddle1, graphics::DrawParam::default())?;
        graphics::draw(ctx, &paddle2, graphics::DrawParam::default())?;
        graphics::draw(ctx, &ball, graphics::DrawParam::default())?;
        
        let score_text = format!("Player 1: {}  Player 2: {}", self.player1_score, self.player2_score);
        let score_display = Text::new((score_text, Font::default(), 48.0));
        let dimensions = score_display.dimensions(ctx);
        graphics::draw(ctx, &score_display, (Point2 { x: (SCREEN_WIDTH - dimensions.w as f32) / 2.0, y: 20.0 }, Color::WHITE))?;

        graphics::present(ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("pong", "Me");
    let (ctx, event_loop) = cb.build()?;
    let state = GameState::new()?;
    event::run(ctx, event_loop, state)
}
