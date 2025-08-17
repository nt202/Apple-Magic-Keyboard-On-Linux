use evdev::{AttributeSet, Device, InputEvent, InputEventKind, Key};

use std::fs;

fn find_magic_keyboard_event_path() -> Option<String> {
    let by_id_path = "/dev/input/by-id";

    // Look for a symlink containing "Magic" and "kbd" (keyboard)
    if let Ok(entries) = fs::read_dir(by_id_path) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.contains("Magic") && file_name.contains("kbd") {
                    // Resolve the symlink to get the actual `/dev/input/eventX` path
                    if let Ok(canonical_path) = fs::canonicalize(entry.path()) {
                        return Some(canonical_path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    None // Not found
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Apple Magic Keyboard On Linux...");

    let event_path = find_magic_keyboard_event_path().ok_or("Magic Keyboard not found in /dev/input/by-id/")?;

    println!("Found Magic Keyboard at: {}", event_path);
    let mut device = Device::open(&event_path)?;
    
    println!("Opened input device: {}", device.name().unwrap_or("unknown"));
    
    device.grab()?; // Grab the device for exclusive access
    println!("Successfully grabbed exclusive access to the input device");
    
    // Create a keyset for the virtual device
    let mut keys = AttributeSet::<Key>::new();

    for code in 1..248 {
        keys.insert(Key::new(code));
    }

    // Configure virtual device
    println!("Creating virtual uinput device...");
    let mut uinput = evdev::uinput::VirtualDeviceBuilder::new()?
        .name("Remapped Keyboard")
        .with_keys(&keys)?
        .build()?;
    println!("Virtual device created successfully!");

    // Fn state
    let mut fn_pressed = false;
    println!("Entering main event loop... (Press Ctrl+C to exit)");

    // Main event loop
    loop {
        match device.fetch_events() {
            Ok(events) => {
                for event in events {
                    // println!("\nReceived event: {:?}", event);
                    
                    if let InputEventKind::Key(key) = event.kind() {
                        // println!("Key event: {:?} {}", key, 
                        //     if event.value() == 1 { "PRESSED" } 
                        //     else if event.value() == 0 { "RELEASED" } 
                        //     else { "REPEAT" });

                        match key {
                            Key::KEY_CAPSLOCK => {
                                uinput.emit(&[InputEvent::new(
                                    event.event_type(),
                                    Key::KEY_ESC.code(),
                                    event.value(),
                                )])?;
                            }

                            Key::KEY_ESC => {
                                uinput.emit(&[InputEvent::new(
                                    event.event_type(),
                                    Key::KEY_CAPSLOCK.code(),
                                    event.value(),
                                )])?;
                            }

                            Key::KEY_102ND => {
                                uinput.emit(&[InputEvent::new(
                                    event.event_type(),
                                    Key::KEY_GRAVE.code(),
                                    event.value(),
                                )])?;
                            }
                            Key::KEY_GRAVE => {
                                uinput.emit(&[InputEvent::new(
                                    event.event_type(),
                                    Key::KEY_102ND.code(),
                                    event.value(),
                                )])?;
                            }

                            // Fn modifier
                            Key::KEY_FN => {
                                fn_pressed = event.value() != 0;
                                // println!("Fn key {}", if fn_pressed { "PRESSED" } else { "RELEASED" });
                            }

                            // HJKL arrow keys when Fn is pressed
                            Key::KEY_H if fn_pressed => {
                                // println!("Remapping Fn+H to LEFT");
                                uinput.emit(&[InputEvent::new(
                                    event.event_type(),
                                    Key::KEY_LEFT.code(),
                                    event.value(),
                                )])?;
                            }
                            Key::KEY_J if fn_pressed => {
                                // println!("Remapping Fn+J to DOWN");
                                uinput.emit(&[InputEvent::new(
                                    event.event_type(),
                                    Key::KEY_DOWN.code(),
                                    event.value(),
                                )])?;
                            }
                            Key::KEY_K if fn_pressed => {
                                // println!("Remapping Fn+K to UP");
                                uinput.emit(&[InputEvent::new(
                                    event.event_type(),
                                    Key::KEY_UP.code(),
                                    event.value(),
                                )])?;
                            }
                            Key::KEY_L if fn_pressed => {
                                // println!("Remapping Fn+L to RIGHT");
                                uinput.emit(&[InputEvent::new(
                                    event.event_type(),
                                    Key::KEY_RIGHT.code(),
                                    event.value(),
                                )])?;
                            }

                            // Command modifier
                            Key::KEY_LEFTMETA => {
                                uinput.emit(&[InputEvent::new(
                                    event.event_type(),
                                    Key::KEY_LEFTCTRL.code(),
                                    event.value(),
                                )])?;
                            }

                            // Pass through other keys unchanged
                            _ => {
                                // println!("Passing through key: {:?}", key);
                                uinput.emit(&[InputEvent::new(
                                    event.event_type(),
                                    event.code(),
                                    event.value(),
                                )])?;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading events: {}", e);
            }
        }
    }
}
