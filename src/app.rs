pub struct BiomorphApp {
    label: String,
    columns: u8,
    rows: u8,
}

impl Default for BiomorphApp {
    fn default() -> Self {
        Self {
            label: "Biomorph evolve".to_owned(),
            columns: 3,
            rows: 3,
        }
    }
}

impl BiomorphApp {
    /// Called once before the first frame.
    pub fn new() -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Default::default()
    }
}

impl eframe::App for BiomorphApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Welcome to Biomorph evolve");

            ui.add(egui::Slider::new(&mut self.columns, 2..=5).text("Columns"));
            ui.add(egui::Slider::new(&mut self.rows, 2..=5).text("Rows"));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Created by");
                    ui.hyperlink_to(" ScratchX98", "https://scratchx98.github.io");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

        });
    }
}
