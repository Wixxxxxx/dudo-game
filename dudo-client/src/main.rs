use macroquad::prelude::*;

struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: &'static str,
}

impl Button {
    fn new(x: f32, y: f32, width: f32, height: f32, text: &'static str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            text,
        }
    }

    fn is_clicked(&self, mouse_x: f32, mouse_y: f32) -> bool {
        mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height
    }

    fn is_hovered(&self, mouse_x: f32, mouse_y: f32) -> bool {
        self.is_clicked(mouse_x, mouse_y)
    }

    fn draw(&self, mouse_x: f32, mouse_y: f32) {
        let button_color = if self.is_hovered(mouse_x, mouse_y) {
            color_u8!(90, 150, 200, 255) // Lighter blue on hover
        } else {
            color_u8!(70, 130, 180, 255) // Steel blue
        };

        // Draw button background with shadow
        draw_rectangle(
            self.x + 4.0,
            self.y + 4.0,
            self.width,
            self.height,
            color_u8!(0, 0, 0, 100),
        );
        draw_rectangle(self.x, self.y, self.width, self.height, button_color);

        // Draw button border
        draw_rectangle_lines(self.x, self.y, self.width, self.height, 3.0, WHITE);

        // Draw button text (centered)
        let text_params = TextParams {
            font_size: 40,
            color: WHITE,
            ..Default::default()
        };
        let text_dims = measure_text(self.text, None, text_params.font_size, 1.0);
        let text_x = self.x + (self.width - text_dims.width) / 2.0;
        let text_y = self.y + (self.height + text_dims.height) / 2.0 - 5.0;
        draw_text_ex(self.text, text_x, text_y, text_params);
    }
}

#[macroquad::main("Dudo - Liar's Dice")]
async fn main() {
    let start_button = Button::new(300.0, 400.0, 200.0, 70.0, "START");
    let mut show_dice = false;
    let mut time = 0.0f32;

    // Load dice texture (path is relative to where you run cargo from - the workspace root)
    let dice_texture = load_texture("dudo-client/src/assets/sprites/dice/die_6_iso.png")
        .await
        .expect(
            "Failed to load dice texture! Make sure dudo-client/src/assets/sprites/dice/die_6_iso.png exists",
        );
    dice_texture.set_filter(FilterMode::Nearest);

    loop {
        time += get_frame_time();

        // Sky blue background
        clear_background(color_u8!(135, 206, 235, 255));

        if !show_dice {
            // Main menu screen

            // Draw flashing title with pulsing effect
            let pulse = (time * 2.0).sin() * 0.3 + 0.7; // Oscillates between 0.4 and 1.0
            let title_color = Color::new(1.0, 1.0, 1.0, pulse);

            // Draw title shadow
            let shadow_offset = 5.0;
            draw_text(
                "DUDO",
                250.0 + shadow_offset,
                150.0 + shadow_offset,
                120.0,
                color_u8!(0, 0, 0, 80),
            );

            // Draw main title with pulsing effect
            let title_params = TextParams {
                font_size: 120,
                color: title_color,
                ..Default::default()
            };
            draw_text_ex("DUDO", 250.0, 150.0, title_params);

            // Draw subtitle
            let subtitle_params = TextParams {
                font_size: 30,
                color: color_u8!(255, 255, 255, 200),
                ..Default::default()
            };
            draw_text_ex("Liar's Dice", 310.0, 190.0, subtitle_params);

            // Draw decorative dice icons (optional embellishment)
            let icon_y = 250.0;
            for i in 0..3 {
                let x = 150.0 + i as f32 * 200.0;
                let bounce = (time * 3.0 + i as f32).sin() * 10.0;
                draw_circle(x, icon_y + bounce, 15.0, WHITE);
            }

            // Draw start button
            let (mouse_x, mouse_y) = mouse_position();
            start_button.draw(mouse_x, mouse_y);

            // Check for button click
            if is_mouse_button_pressed(MouseButton::Left) {
                if start_button.is_clicked(mouse_x, mouse_y) {
                    show_dice = true;
                }
            }
        } else {
            // Show dice screen

            // Draw dice centered (adjust scale to fit on screen)
            let dice_scale = 0.75; // Reduced from 4.0 to fit better
            let dice_x = (screen_width() - dice_texture.width() * dice_scale) / 2.0;
            let dice_y = (screen_height() - dice_texture.height() * dice_scale) / 2.0;

            draw_texture_ex(
                &dice_texture,
                dice_x,
                dice_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        dice_texture.width() * dice_scale,
                        dice_texture.height() * dice_scale,
                    )),
                    ..Default::default()
                },
            );

            // Draw instruction text
            draw_text(
                "Dice displayed! Press ESC to go back.",
                200.0,
                50.0,
                30.0,
                WHITE,
            );

            // Allow going back to menu
            if is_key_pressed(KeyCode::Escape) {
                show_dice = false;
            }
        }

        next_frame().await;
    }
}
