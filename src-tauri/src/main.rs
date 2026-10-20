// PokeBonk Desktop — contenedor mínimo para pokebonk.com
//
// Filosofía de diseño: este proceso hace LO MÍNIMO INDISPENSABLE.
//   - Una sola ventana que abre directo el juego (nada de páginas intermedias)
//   - Cero plugins, cero IPC, cero telemetría, cero logging a disco
//   - Cero temporizadores, cero hilos extra, cero polling
//   - Sin persistencia propia: no escribe NADA en tu disco
//   - Al cerrar la ventana, el proceso termina y libera toda su memoria
//
// El script INIT_JS agrega dos seguros de vida (consumo: 2 listeners, ~0 KB):
//   1) F5 / Ctrl+R recargan el juego (recuperación rápida tras un cuelgue)
//   2) webglcontextlost + preventDefault: permite que el motor restaure el
//      contexto WebGL en vez de morir (práctica estándar para juegos WebGL
//      cuando la memoria de video se agota — justo el caso de una APU).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const INIT_JS: &str = r#"
(function () {
  if (window.__pokebonk_init__) return;
  window.__pokebonk_init__ = true;

  // Recuperación rápida: F5 o Ctrl+R recargan el juego.
  window.addEventListener('keydown', function (e) {
    if (e.key === 'F5' ||
        (e.ctrlKey && !e.shiftKey && !e.altKey && (e.key === 'r' || e.key === 'R'))) {
      e.preventDefault();
      window.location.reload();
    }
  }, true);

  // Si el contexto WebGL se pierde por falta de memoria de video, darle al
  // motor la oportunidad de restaurarlo automáticamente.
  window.addEventListener('webglcontextlost', function (e) {
    e.preventDefault();
  }, false);
})();
"#;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Ventana única, directa al juego. WebView2 (motor nativo de
            // Windows, el mismo de Edge) ya viene preinstalado en Windows 10.
            tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External(tauri::Url::parse("https://pokebonk.com/")?),
            )
            .title("PokeBonk")
            .inner_size(1280.0, 720.0)
            .min_inner_size(640.0, 360.0)
            .resizable(true)
            .center()
            .initialization_script(INIT_JS)
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
