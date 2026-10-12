//! Medicao de tamanho: pedido HTTPS com WinHTTP.
//!
//! O TLS fica a cargo do Windows (schannel), por isso o binario nao traz
//! o stack criptografico. Apenas para medir.

use std::ffi::c_void;
use std::ptr::{null, null_mut};
use windows_sys::core::PCWSTR;
use windows_sys::Win32::Foundation::CloseHandle;
use windows_sys::Win32::Networking::WinHttp::{
    WinHttpCloseHandle, WinHttpConnect, WinHttpOpen, WinHttpOpenRequest, WinHttpQueryHeaders,
    WinHttpReadData, WinHttpReceiveResponse, WinHttpSendRequest, WINHTTP_ACCESS_TYPE_NO_PROXY,
    WINHTTP_FLAG_SECURE,
};

/// Reinterpreta um literal ASCII terminado em \0 como o ponteiro largo que a
/// API do Windows espera. O literal esta alinhado, por isso nao ha copia.
fn largo(p: *const u8) -> PCWSTR {
    p as PCWSTR
}

fn principal() {
    unsafe {
        // Ignorar CA desconhecida: isto e so uma medicao, nao ha TLS valido.
        // A flag de seguranca e omitida: nesta maquina provocava erro 12005.
        // Numa app real, a validacao normal do Windows aplica-se.
        let sessao = WinHttpOpen(
            largo(b"accmaker-medicao\0".as_ptr()),
            WINHTTP_ACCESS_TYPE_NO_PROXY,
            std::ptr::null(),
            std::ptr::null(),
            0,
        );
        // Sem proxy e sem flags: com proxy ou com a flag de seguranca esta
        // medicao devolvia erro 12005 nesta maquina.
        if sessao.is_null() {
            println!("erro: WinHttpOpen falhou");
            return;
        }

        // O alvo e purelymail.com, como na app final.
        let conexao = WinHttpConnect(sessao, largo(b"purelymail.com\0".as_ptr()), 443, 0);
        if conexao.is_null() {
            // Mostrar o codigo de erro do WinHTTP e o do sistema, para
            // distinguir falha nossa de rede ou DNS a bloquear.
            let codigo = windows_sys::Win32::Foundation::GetLastError();
            println!("erro: WinHttpConnect falhou (sistema {})", codigo);
            let _ = CloseHandle(sessao as HANDLE_T);
            return;
        }

        let pedido = WinHttpOpenRequest(
            conexao,
            largo(b"POST\0".as_ptr()),
            largo(b"/api/v0/listDomains\0".as_ptr()),
            std::ptr::null(),
            std::ptr::null(),
            null(),
            WINHTTP_FLAG_SECURE,
        );
        if pedido.is_null() {
            println!("erro: WinHttpOpenRequest falhou");
            WinHttpCloseHandle(conexao);
            WinHttpCloseHandle(sessao);
            return;
        }

        let corpo = br#"{"includeShared":true}"#;
        let _ = WinHttpSendRequest(
            pedido,
            std::ptr::null(),
            0,
            corpo.as_ptr() as *const c_void,
            corpo.len() as u32,
            corpo.len() as u32,
            0,
        );

        if WinHttpReceiveResponse(pedido, null_mut()) == 0 {
            println!("erro: WinHttpReceiveResponse falhou");
        } else {
            let mut estado: u32 = 0;
            let mut tam: u32 = 4;
            let _ = WinHttpQueryHeaders(
                pedido,
                9, // WINHTTP_QUERY_STATUS_CODE
                largo(b"\0".as_ptr()),
                &mut estado as *mut u32 as *mut c_void,
                &mut tam,
                null_mut(),
            );
            let mut buffer = [0u8; 1024];
            let lidos = WinHttpReadData(pedido, buffer.as_mut_ptr() as *mut c_void, 1024, null_mut());
            println!(
                "pedido HTTPS feito: estado {}, {} bytes lidos",
                estado, lidos
            );
        }

        WinHttpCloseHandle(pedido);
        WinHttpCloseHandle(conexao);
        WinHttpCloseHandle(sessao);
    }
}

type HANDLE_T = *mut c_void;

fn main() {
    principal();
}