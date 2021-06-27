fn main() {
    windows::build! {
        Windows::Win32::UI::Magnification::*,
        Windows::Win32::UI::KeyboardAndMouseInput::*,
        Windows::Win32::UI::WindowsAndMessaging::*,
        Windows::Win32::System::Console::FreeConsole,
    }
}
