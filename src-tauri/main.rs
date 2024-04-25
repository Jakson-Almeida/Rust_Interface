fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Configurações iniciais podem ser feitas aqui
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Lista de comandos que você deseja expor ao frontend
        ])
        .run(tauri::generate_context!())
        .expect("erro ao rodar a aplicação Tauri");
}
