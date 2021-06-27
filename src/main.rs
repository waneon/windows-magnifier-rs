use bindings::Windows::Win32::{
    Foundation::POINT, System::Console::FreeConsole, UI::KeyboardAndMouseInput::*,
    UI::Magnification::*, UI::WindowsAndMessaging::*,
};

static MOD: HOT_KEY_MODIFIERS = MOD_ALT;
static KEY: u32 = b'1' as u32;
static MAGNIFYING_FACTOR: f32 = 2.0;
static TIMER_MS: u32 = 8;

struct MagInfo {
    mul_x: f32,
    mul_y: f32,
    sub_x: i32,
    sub_y: i32,
    max_x: i32,
    max_y: i32,
}

impl MagInfo {
    fn init() -> MagInfo {
        let x: i32;
        let y: i32;

        unsafe {
            x = GetSystemMetrics(SM_CXSCREEN);
            y = GetSystemMetrics(SM_CYSCREEN);
        }

        let k = (x as f32 / 9.0) as i32;
        let mul_x = x as f32 * (1.0 - 1.0 / MAGNIFYING_FACTOR) / (x - 2 * k) as f32;
        let mul_y = y as f32 * (1.0 - 1.0 / MAGNIFYING_FACTOR) / (y - 2 * k) as f32;

        MagInfo {
            mul_x,
            mul_y,
            sub_x: (k as f32 * mul_x) as i32,
            sub_y: (k as f32 * mul_y) as i32,
            max_x: ((x as f32) * (1.0 - 1.0 / MAGNIFYING_FACTOR)) as i32,
            max_y: ((y as f32) * (1.0 - 1.0 / MAGNIFYING_FACTOR)) as i32,
        }
    }
}

fn main() {
    let mut magnified = false;
    let info = MagInfo::init();

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
                    magnify_on(&info);
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

fn magnify_on(info: &MagInfo) {
    let mut point = POINT::default();
    let x: i32;
    let y: i32;

    unsafe {
        GetCursorPos(&mut point);
        x = (point.x as f32 * info.mul_x) as i32 - info.sub_x;
        y = (point.y as f32 * info.mul_y) as i32 - info.sub_y;

        // Check bound
        let x = if x < 0 {
            0
        } else if x > info.max_x {
            info.max_x
        } else {
            x
        };
        let y = if y < 0 {
            0
        } else if y > info.max_y {
            info.max_y
        } else {
            y
        };

        MagSetFullscreenTransform(MAGNIFYING_FACTOR, x, y);
    }
}

fn magnify_off() {
    unsafe {
        MagSetFullscreenTransform(1.0, 0, 0);
    }
}
