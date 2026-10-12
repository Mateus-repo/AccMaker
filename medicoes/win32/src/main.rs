// Medição de tamanho: variante Win32 nativa, janela com um botão.
// Apenas windows-sys, sem runtimes nem crates graficos.
// Serve só para medir o .exe, não é a app final.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, RegisterClassW,
    ShowWindow, TranslateMessage, WNDCLASSW, WS_OVERLAPPEDWINDOW,
};

extern "system" fn janela(_h: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            1 => {
                // WM_CREATE: cria o botão
                let classe: Vec<u16> = "BUTTON\0".encode_utf16().collect();
                let texto: Vec<u16> = "Criar\0".encode_utf16().collect();
                CreateWindowExW(
                    0,
                    classe.as_ptr(),
                    texto.as_ptr(),
                    0x00000001 | 0x00C00000 | 0x00040000, // WS_CHILD|WS_VISIBLE|BS_PUSHBUTTON
                    20,
                    60,
                    160,
                    40,
                    _h,
                    1 as *mut _,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
                0
            }
            2 => 0, // WM_DESTROY
            _ => DefWindowProcW(_h, msg, w, l),
        }
    }
}

fn main() {
    unsafe {
        let classe: Vec<u16> = "medicao-win32\0".encode_utf16().collect();
        let titulo: Vec<u16> = "medicao-win32\0".encode_utf16().collect();
        let wc = WNDCLASSW {
            style: 0x0003,          // CS_HREDRAW|CS_VREDRAW
            lpfnWndProc: Some(janela),
            hInstance: GetModuleHandleW(std::ptr::null()),
            lpszClassName: classe.as_ptr(),
            ..std::mem::zeroed()
        };
        RegisterClassW(&wc);
        let hwnd = CreateWindowExW(
            0,
            classe.as_ptr(),
            titulo.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            200,
            200,
            400,
            260,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            wc.hInstance,
            std::ptr::null_mut(),
        );
        ShowWindow(hwnd, 1);
        let mut msg = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}