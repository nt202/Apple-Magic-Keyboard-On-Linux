# Apple Magic Keyboard On Linux  
**Bring macOS-like shortcuts to your Apple Magic Keyboard on Linux**  

Tired of losing your macOS muscle memory on Linux? This tool remaps your **Apple Magic Keyboard** to behave like it does on macOS, with bonus Vim-style `HJKL` arrow keys. No more fighting with defaults!  

![Demo GIF or screenshot could go here]  

## Why This Exists  
- You love macOS shortcuts but use Linux.  
- You want `HJKL` as arrow keys *everywhere* (like Vim).  
- The Magic Keyboard’s `Fn`/`Ctrl`/`Meta` keys don’t work out-of-the-box on Linux.  

## Features  
- **Swap `CapsLock` ↔ `Esc`** (for Vim users).  
- **Remap `Fn + HJKL` to arrow keys** (left/down/up/right).  
- **Fix `§`/`~` key mismaps** (common on non-macOS systems).  
- **Meta (⌘) → Ctrl** for macOS-like shortcuts.  

## Customizing  
Edit `src/main.rs` to tweak key mappings. Need to find your keycodes? Use:  
```sh
sudo evtest  # Look for your keyboard in the list
```  

## Running at Startup  
To launch automatically on boot:  
1. Create a systemd service (example in `contrib/apple-magic-keyboard-on-linux.service`).  
2. Enable it:  
   ```sh
   sudo systemctl enable --now apple-magic-keyboard-on-linux.service
   ```  

## Troubleshooting  
- **Keyboard not found?**  
  Check `/dev/input/by-id/` for a symlink containing `Magic` and `kbd`.  
- **Permissions issue?**  
  Ensure you are running the app with sudo

## Like This? Support the Project  
- ⭐ **Star the repo**  
- 🐞 **Report issues**  
- 🎯 **Contribute code**  
