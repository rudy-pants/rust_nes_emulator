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
        let max_x = u8::MAX - (BALL_SIZE - 1);
        let max_y = NES_HEIGHT - BALL_SIZE;

        self.ball_x = advance_axis(self.ball_x, &mut self.vel_x, max_x);
        self.ball_y = advance_axis(self.ball_y, &mut self.vel_y, max_y);
    }
}

fn advance_axis(position: u8, velocity: &mut u8, max: u8) -> u8 {
    let forward = *velocity & 0x80 == 0;
    let speed = if forward {
        *velocity
    } else {
        velocity.wrapping_neg()
    };
    // Distance to border
    let distance = if forward { max - position } else { position };

    if speed > distance {
        *velocity = velocity.wrapping_neg();
    }

    position.wrapping_add(*velocity)
}

impl eframe::App for NesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_ball();

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let available = ui.available_size();

                // Scale the NES display to fit while maintaining aspect ratio
                let nes_width = f32::from(NES_WIDTH.wrapping_sub(1)) + 1.0;
                let scale = (available.x / nes_width).min(available.y / f32::from(NES_HEIGHT));
                let display_w = nes_width * scale;
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
