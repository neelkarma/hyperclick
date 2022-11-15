use std::{env::args, time::Instant};

use macroquad::prelude::*;

#[macroquad::main("hyperclick")]
async fn main() {
    let time = args().nth(1).and_then(|arg| arg.parse::<i32>().ok());

    let mut start = Instant::now();
    let mut playing = false;
    let mut clicks = 0;

    loop {
        if is_mouse_button_down(MouseButton::Left) {
            clear_background(DARKGRAY);
        } else {
            clear_background(BLACK);
        };

        if is_key_pressed(KeyCode::Q) {
            if !playing {
                return;
            }
            break;
        };

        if is_mouse_button_pressed(MouseButton::Left) {
            clicks += 1;
            if !playing {
                playing = true;
                start = Instant::now();
            }
        }

        if playing {
            let elapsed = start.elapsed().as_secs_f64();
            if let Some(time) = time {
                if elapsed >= time as f64 {
                    break;
                }
            }
            draw_text(&format!("Time: {} sec", elapsed), 10., 20., 18., WHITE);
            draw_text(&format!("Clicks: {}", clicks), 10., 40., 18., WHITE);
            draw_text(
                &format!("CPS: {}", clicks as f64 / elapsed),
                10.,
                60.,
                18.,
                WHITE,
            );
            draw_text("Press Q to quit", 10., 80., 18., WHITE);
        } else {
            draw_text("Click anywhere to start", 10., 20., 18., WHITE);
            draw_text("Press Q to quit", 10., 40., 18., WHITE);
        }

        next_frame().await
    }

    let elapsed = start.elapsed().as_secs_f64();
    println!(
        "{} clicks in {} seconds -> {} CPS",
        clicks,
        elapsed,
        clicks as f64 / elapsed
    );
}
