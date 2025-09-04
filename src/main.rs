use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Initialize logging with more verbose output
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    log::info!("Starting Emulite emulator...");

    // Create the GUI application
    let app = emulite::gui::EmuliteApp::new();

    // Configure the native options
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        min_window_size: Some(egui::vec2(800.0, 600.0)),
        window_builder: Some(Box::new(|builder| {
            builder
                .with_title("Emulite - Multi-platform Video Game Emulator")
        })),
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Emulite",
        native_options,
        Box::new(|_cc| Box::new(app)),
    )
}
