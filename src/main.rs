use bindings::Windows::Win32::{
    Foundation::POINT, System::Console::FreeConsole, UI::KeyboardAndMouseInput::*,
    UI::Magnification::*, UI::WindowsAndMessaging::*,
};

static MOD: HOT_KEY_MODIFIERS = MOD_ALT;
static KEY: u32 = b'1' as u32;
static MAGNIFYING_FACTOR: f32 = 2.0;
static TIMER_MS: u32 = 8;

fn main() {
    let mut magnified = false;

    unsafe {
        // Hide console
        FreeConsole();
        // Init mag
        if MagInitialize().as_bool() != true {
            return;
        }
        // Set toggle key
        if RegisterHotKey(None, 1, MOD, KEY).as_bool() != true {
            return;
        }
        if RegisterHotKey(None, 2, MOD, b'2' as u32).as_bool() != true {
            return;
        }

        // Check event loop
        let mut msg: MSG = MSG::default();
        while GetMessageA(&mut msg, None, 0, 0).as_bool() != false {
            // Quit
            if msg.message == WM_HOTKEY && msg.wParam.0 == 2 {
                break;
            }
            // Toggle Magnifying
            if msg.message == WM_HOTKEY && msg.wParam.0 == 1 {
                if magnified {
                    KillTimer(None, 1);
                    magnify_off();
                } else {
                    SetTimer(None, 1, TIMER_MS, None);
                }
                magnified = !magnified;
            }
            // Turn on magnifying
            if msg.message == WM_TIMER {
                if magnified {
                    magnify_on();
                }
            }
        }

        // Unset toggle key
        UnregisterHotKey(None, 1);
        UnregisterHotKey(None, 2);
        // Free mag
        MagUninitialize();
    }
}

fn magnify_on() {
    let mut point = POINT::default();

    unsafe {
        // Get cursor point
        GetCursorPos(&mut point);

        MagSetFullscreenTransform(
            MAGNIFYING_FACTOR,
            (point.x as f32 * (1.0 / MAGNIFYING_FACTOR)) as i32,
            (point.y as f32 * (1.0 / MAGNIFYING_FACTOR)) as i32,
        );
    }
}

fn magnify_off() {
    unsafe {
        MagSetFullscreenTransform(1.0, 0, 0);
    }
}
