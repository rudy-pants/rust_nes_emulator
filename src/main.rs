use eframe::egui;
use egui::{Color32, Pos2, Rect, Vec2};

const NES_WIDTH: f32 = 256.0;
const NES_HEIGHT: f32 = 240.0;
const BALL_SIZE: f32 = 2.0;
const BALL_SPEED: f32 = 1.5; // pixels per frame

struct NesApp {
    ball_x: f32,
    ball_y: f32,
    vel_x: f32,
    vel_y: f32,
}

impl NesApp {
    fn new() -> Self {
        Self {
            ball_x: 0.0,
            ball_y: 0.0,
            vel_x: BALL_SPEED,
            vel_y: BALL_SPEED,
        }
    }

    fn update_ball(&mut self) {
        self.ball_x += self.vel_x;
        self.ball_y += self.vel_y;

        // Bounce off right/left walls
        if self.ball_x + BALL_SIZE >= NES_WIDTH {
            self.ball_x = NES_WIDTH - BALL_SIZE;
            self.vel_x = -self.vel_x;
        } else if self.ball_x <= 0.0 {
            self.ball_x = 0.0;
            self.vel_x = -self.vel_x;
        }

        // Bounce off bottom/top walls
        if self.ball_y + BALL_SIZE >= NES_HEIGHT {
            self.ball_y = NES_HEIGHT - BALL_SIZE;
            self.vel_y = -self.vel_y;
        } else if self.ball_y <= 0.0 {
            self.ball_y = 0.0;
            self.vel_y = -self.vel_y;
        }
    }
}

impl eframe::App for NesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_ball();

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let available = ui.available_size();

                // Scale the NES display to fit while maintaining aspect ratio
                let scale = (available.x / NES_WIDTH).min(available.y / NES_HEIGHT);
                let display_w = NES_WIDTH * scale;
                let display_h = NES_HEIGHT * scale;

                // Center the display
                let offset_x = (available.x - display_w) / 2.0;
                let offset_y = (available.y - display_h) / 2.0;

                let painter = ui.painter();
                let panel_rect = ui.max_rect();
                let origin = panel_rect.min + Vec2::new(offset_x, offset_y);

                // Draw black NES screen background
                painter.rect_filled(
                    Rect::from_min_size(origin, Vec2::new(display_w, display_h)),
                    0.0,
                    Color32::BLACK,
                );

                // Draw the ball (scaled to display)
                let ball_rect = Rect::from_min_size(
                    Pos2::new(
                        origin.x + self.ball_x * scale,
                        origin.y + self.ball_y * scale,
                    ),
                    Vec2::splat(BALL_SIZE * scale),
                );
                painter.rect_filled(ball_rect, 0.0, Color32::WHITE);
            });

        // Request continuous repaint for animation
        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("NES Emulator")
            .with_inner_size([512.0, 480.0]), // 2x NES resolution
        ..Default::default()
    };

    eframe::run_native(
        "NES Emulator",
        options,
        Box::new(|_cc| Ok(Box::new(NesApp::new()))),
    )
}
