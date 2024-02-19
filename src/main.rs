use ctrlc;
use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use std::time::{Duration, Instant};

fn enter_idle_state(device_state: &DeviceState, last_mouse_position: &mut Option<MouseState>) {
    println!("Entering idle state.");
    println!("Starting screensaver.");

    loop {
        let mouse = device_state.get_mouse();
        let keys: Vec<Keycode> = device_state.get_keys();

        // Check for mouse movement or button press
        let mouse_moved = last_mouse_position.as_ref().map_or(false, |last| last.coords != mouse.coords);
        let mouse_clicked = mouse.button_pressed.iter().any(|&pressed| pressed);

        if !keys.is_empty() || mouse_moved || mouse_clicked {
            println!("Input detected. Exiting screensaver.");
            break;
        }

        *last_mouse_position = Some(mouse);
        std::thread::sleep(Duration::from_millis(100)); // Check for input every 100 milliseconds
    }
}

fn main() {
    // Set up the Ctrl+C handler
    ctrlc::set_handler(move || {
        println!("\nExiting gracefully...");
        std::process::exit(0);
    }).expect("Error setting Ctrl+C handler");

    let device_state = DeviceState::new();
    let mut last_mouse_position: Option<MouseState> = None;

    println!("Countdown from 10 to 1:");

    loop {
        let mut countdown = 10;
        let mut countdown_start = Instant::now();

        while countdown > 0 {
            let start_time = Instant::now();
            let mut input_detected = false;

            println!("{}", countdown);

            while Instant::now() - start_time < Duration::from_secs(1) {
                let mouse = device_state.get_mouse();

                if Instant::now() - countdown_start >= Duration::from_secs(2) {
                    let keys: Vec<Keycode> = device_state.get_keys();

                    // Check for mouse movement or button press
                    let mouse_moved = last_mouse_position.as_ref().map_or(false, |last| last.coords != mouse.coords);
                    let mouse_clicked = mouse.button_pressed.iter().any(|&pressed| pressed);

                    if !keys.is_empty() || mouse_moved || mouse_clicked {
                        input_detected = true;
                        break;
                    }
                }

                last_mouse_position = Some(mouse);
            }

            if input_detected {
                println!("Input detected. Restarting countdown.");
                countdown = 10; // Restart the countdown
                countdown_start = Instant::now(); // Reset the countdown start time
            } else {
                countdown -= 1;
            }
        }

        // Enter idle state and start screensaver when countdown finishes
        enter_idle_state(&device_state, &mut last_mouse_position);
    }
}
