use eframe::egui;
use egui::{Color32, Pos2, Rect, Vec2};

const NES_WIDTH: u8 = 0; // cannot use 256 because it is too large for u8
const NES_HEIGHT: u8 = 240;
const BALL_SIZE: u8 = 2;
const BALL_SPEED: u8 = 1; // pixels per frame

struct NesApp {
    ball_x: u8,
    ball_y: u8,
    vel_x: u8,
    vel_y: u8,
}

impl NesApp {
    fn new() -> Self {
        Self {
            ball_x: 0,
            ball_y: 0,
            vel_x: BALL_SPEED,
            vel_y: BALL_SPEED,
        }
    }

    fn update_ball(&mut self) {
        println!("{}, {}", self.ball_x, self.ball_y);
        let mut next_ball_x = self.ball_x.wrapping_add(self.vel_x);
        let mut next_ball_y = self.ball_y.wrapping_add(self.vel_y);

        // Bounce off right/left walls
        // Because NES WIDTH is 256, simply check if overflow happens
        if (self.ball_x & 0x80) != (next_ball_x & 0x80) {
            self.vel_x = self.vel_x.wrapping_neg();
            next_ball_x = self.ball_x.wrapping_add(self.vel_x);
        }

        // Bounce off bottom/top walls
        if ((self.ball_y < NES_HEIGHT) && (next_ball_y >= NES_HEIGHT))
            || ((self.ball_y & 0x80) != (next_ball_y & 0x80))
        {
            self.vel_y = self.vel_y.wrapping_neg();
            next_ball_y = self.ball_y.wrapping_add(self.vel_y);
        }

        self.ball_x = next_ball_x;
        self.ball_y = next_ball_y;
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
                let scale =
                    (available.x / f32::from(NES_WIDTH)).min(available.y / f32::from(NES_HEIGHT));
                let display_w = f32::from(NES_WIDTH) * scale;
                let display_h = f32::from(NES_HEIGHT) * scale;

                let painter = ui.painter();
                let panel_rect = ui.max_rect();
                let origin = panel_rect.min;

                // Draw black NES screen background
                painter.rect_filled(
                    Rect::from_min_size(origin, Vec2::new(display_w, display_h)),
                    0.0,
                    Color32::BLACK,
                );

                // Draw the ball (scaled to display)
                let ball_rect = Rect::from_min_size(
                    Pos2::new(
                        origin.x + f32::from(self.ball_x) * scale,
                        origin.y + f32::from(self.ball_y) * scale,
                    ),
                    Vec2::splat(f32::from(BALL_SIZE) * scale),
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
