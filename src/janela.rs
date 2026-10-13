//! Janela Win32 do AccMaker.
//!
//! Nativa de proposito: a janela com `egui` media 2,85 MB contra 0,11 MB
//! desta. Nao ha animacoes nem barra de forca — o que o Windows da de graca.
//!
//! Decisoes que o teste com conta real fixou:
//! - a chave mostra-se em claro (quem a tem recebe 403 ao apagar a inbox);
//! - apagar e um botao, por baixo sao tres pedidos (as keys, depois a inbox);
//! - username duplicado da mensagem "Esse username ja esta em uso".

use std::ptr;

use windows_sys::Win32::Foundation::{RECT, HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{
    BeginPaint, CreateFontIndirectW, CreatePen, DeleteObject, EndPaint, FillRect, FrameRect,
    GetStockObject, SelectObject, SetBkMode, SetTextColor, TextOutW, UpdateWindow, HDC,
    HGDIOBJ, HFONT, LOGFONTW, LOGPEN, PAINTSTRUCT, PS_SOLID, TRANSPARENT,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

/// Layout: tres slots, cada um com um botao.
const SLOTS: usize = 3;

/// Altura de cada slot, em unidades de janela.
const ALTURA_SLOT: i32 = 56;
const MARGEM: i32 = 12;
const ALTURA_BOTAO: i32 = 26;
const LARGURA_BOTAO: i32 = 84;

/// Estado da janela, do ponto de vista do desenho.
pub struct Estado {
    /// Email de cada slot, `None` se a vaga esta livre.
    pub emails: [Option<String>; SLOTS],
    /// Estado da accao em curso, mostrado em baixo.
    pub mensagem: String,
}

impl Estado {
    pub fn novo() -> Self {
        Estado {
            emails: [None, None, None],
            mensagem: String::new(),
        }
    }

    /// Le os slots a partir da lista da API, que e a fonte da verdade.
    pub fn aplicar(&mut self, contas: &[crate::provider::Inbox]) {
        self.emails = [None, None, None];
        for (i, conta) in contas.iter().take(SLOTS).enumerate() {
            self.emails[i] = Some(conta.email.clone());
        }
    }

    pub fn ocupadas(&self) -> usize {
        self.emails.iter().filter(|e| e.is_some()).count()
    }
}

impl Default for Estado {
    fn default() -> Self {
        Estado::novo()
    }
}

/// Converte texto para o buffer largo que a API do Windows espera.
fn largo(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_CREATE => {
                let estado = Box::new(Estado::novo());
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(estado) as isize);
                1
            }
            WM_PAINT => {
                let estado = &*(GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const Estado);
                pintar(hwnd, estado);
                1
            }
            WM_DESTROY => {
                let p = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Estado;
                if !p.is_null() {
                    drop(Box::from_raw(p));
                }
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcW(hwnd, msg, w, l),
        }
    }
}

/// Desenha os tres slots e a mensagem de estado.
unsafe fn pintar(hwnd: HWND, estado: &Estado) {
    let mut ps: PAINTSTRUCT = std::mem::zeroed();
    let hdc = BeginPaint(hwnd, &mut ps);

    let fundo = GetStockObject(15); // COLOR_WINDOW
    let _anterior = SelectObject(hdc, fundo);
    FillRect(hdc, &ps.rcPaint, fundo);

    SetBkMode(hdc, TRANSPARENT as i32);
    SetTextColor(hdc, 0x00302010); // RGB(16, 32, 48)

    let fonte: HFONT = criar_fonte(14);
    let _anterior = SelectObject(hdc, fonte);

    // Cabecalho
    let titulo = largo("accmaker");
    TextOutW(
        hdc,
        MARGEM,
        10,
        titulo.as_ptr(),
        titulo.len() as i32 - 1,
    );

    // Slots
    for (i, email) in estado.emails.iter().enumerate() {
        let topo = 40 + i as i32 * ALTURA_SLOT;
        let rect = RECT {
            left: MARGEM,
            top: topo,
            right: 400,
            bottom: topo + ALTURA_SLOT - 8,
        };
        let traco = criar_caneta();
        let traco_ant = SelectObject(hdc, traco);
        FrameRect(hdc, &rect, traco);
        SelectObject(hdc, traco_ant);
        DeleteObject(traco);

        let texto = match email {
            Some(e) => e.clone(),
            None => "vaga livre".to_string(),
        };
        let linha = largo(&texto);
        TextOutW(hdc, MARGEM + 10, topo + 8, linha.as_ptr(), linha.len() as i32 - 1);
    }

    // Rodape: estado
    let rodape = largo(&format!(
        "{}/{} contas   {}",
        estado.ocupadas(),
        SLOTS,
        estado.mensagem
    ));
    TextOutW(
        hdc,
        MARGEM,
        40 + SLOTS as i32 * ALTURA_SLOT + 8,
        rodape.as_ptr(),
        rodape.len() as i32 - 1,
    );

    SelectObject(hdc, _anterior);
    DeleteObject(fonte);
    EndPaint(hwnd, &ps);
}

unsafe fn criar_fonte(tam: i32) -> HFONT {
    let mut fonte: LOGFONTW = std::mem::zeroed();
    fonte.lfHeight = -tam;
    fonte.lfWeight = 400;
    fonte.lfCharSet = 1; // DEFAULT_CHARSET
    let mut nome16 = [0u16; 32];
    for (i, c) in "Segoe UI".encode_utf16().take(31).enumerate() {
        nome16[i] = c;
    }
    fonte.lfFaceName = nome16;
    CreateFontIndirectW(&fonte)
}

unsafe fn criar_caneta() -> HGDIOBJ {
    // Caneta de 1 px, RGB(64, 96, 160).
    CreatePen(PS_SOLID, 1, 0x00A06040)
}

/// Abre a janela. `estilo_subsistema` controla se corre com consola.
pub fn abrir(titulo: &str) {
    unsafe {
        let classe = RegisterClassW(&WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            hInstance: GetModuleHandleW(ptr::null()),
            lpszClassName: largo("accmaker").as_ptr(),
            ..std::mem::zeroed()
        });
        if classe == 0 {
            return;
        }
        let hwnd = CreateWindowExW(
            0,
            largo("accmaker").as_ptr(),
            largo(titulo).as_ptr(),
            WS_OVERLAPPEDWINDOW,
            200,
            150,
            430,
            40 + SLOTS as i32 * ALTURA_SLOT + 70,
            ptr::null_mut(),
            ptr::null_mut(),
            GetModuleHandleW(ptr::null()),
            ptr::null_mut(),
        );
        if hwnd.is_null() {
            return;
        }
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
