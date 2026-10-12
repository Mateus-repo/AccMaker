// Medição de tamanho: variante egui, janela com um botão.
// Serve apenas para medir o .exe, não é a app final.

fn main() -> eframe::Result<()> {
    let opcoes = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([360.0, 200.0]),
        ..Default::default()
    };
    eframe::run_native(
        "medicao-egui",
        opcoes,
        Box::new(|_cc| Ok(Box::new(Mostrar))),
    )
}

struct Mostrar;

impl eframe::App for Mostrar {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("medicao-egui");
            if ui.button("Criar").clicked() {
                // apenas um botao, sem logica
            }
        });
    }
}