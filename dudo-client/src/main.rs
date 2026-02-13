use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Canvas, Color, DrawParam, Rect, Text};
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};

struct MainMenuState {
    start_button_rect: Rect,
}

impl MainMenuState {
    fn new(_ctx: &mut Context) -> GameResult<MainMenuState> {
        let button_width = 200.0;
        let button_height = 60.0;
        let button_x = 400.0 - button_width / 2.0;
        let button_y = 400.0;

        Ok(MainMenuState {
            start_button_rect: Rect::new(button_x, button_y, button_width, button_height),
        })
    }
}

impl EventHandler for MainMenuState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create canvas with sky blue background
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(135, 206, 235));

        // Draw title
        let title = Text::new("DUDO");
        canvas.draw(
            &title,
            DrawParam::default()
                .dest(Point2 { x: 320.0, y: 150.0 })
                .scale([3.0, 3.0]),
        );

        // Draw start button background
        let button_bg = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.start_button_rect,
            Color::from_rgb(70, 130, 180),
        )?;
        canvas.draw(&button_bg, DrawParam::default());

        // Draw button border
        let button_border = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(3.0),
            self.start_button_rect,
            Color::WHITE,
        )?;
        canvas.draw(&button_border, DrawParam::default());

        // Draw button text
        let button_text = Text::new("START");
        let text_x = self.start_button_rect.x + 60.0;
        let text_y = self.start_button_rect.y + 15.0;
        canvas.draw(
            &button_text,
            DrawParam::default().dest(Point2 {
                x: text_x,
                y: text_y,
            }),
        );

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {
            if self.start_button_rect.contains(Point2 { x, y }) {
                println!("Start button clicked!");
                ctx.request_quit();
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("dudo", "author")
        .window_setup(WindowSetup::default().title("Dudo - Liar's Dice"))
        .window_mode(WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let state = MainMenuState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
