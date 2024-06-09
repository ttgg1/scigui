use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native("SciGui", options, Box::new(|_cc| Box::<MyApp>::default()))
}
#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut plot_rect = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Save Plot").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
            }

            if ui.button("Load Data").clicked() {
                println!("Pressed Load");
                let file = rfd::FileDialog::new()
                    .add_filter("text-file", &["txt"])
                    .add_filter("CSV-file", &["csv", "CSV"])
                    .set_directory(".")
                    .pick_file();

                // TODO: load file
                match file {
                    Some(file) => println!("Picked file {file:?}"),
                    None => eprintln!("Could not load File!"),
                }
            }

            let my_plot = Plot::new("My Plot").legend(Legend::default());

            // let's create a dummy line in the plot
            let graph: Vec<[f64; 2]> = vec![[0.0, 1.0], [2.0, 3.0], [3.0, 2.0]];
            let inner = my_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from(graph)).name("curve"));
            });
            // Remember the position of the plot
            plot_rect = Some(inner.response.rect);
        });

        // Check for returned screenshot:
        let screenshot = ctx.input(|i| {
            for event in &i.raw.events {
                if let egui::Event::Screenshot { image, .. } = event {
                    return Some(image.clone());
                }
            }
            None
        });

        if let (Some(screenshot), Some(plot_location)) = (screenshot, plot_rect) {
            if let Some(mut path) = rfd::FileDialog::new().save_file() {
                path.set_extension("png");

                // for a full size application, we should put this in a different thread,
                // so that the GUI doesn't lag during saving

                let pixels_per_point = ctx.pixels_per_point();
                let plot = screenshot.region(&plot_location, Some(pixels_per_point));
                // save the plot to png
                image::save_buffer(
                    &path,
                    plot.as_raw(),
                    plot.width() as u32,
                    plot.height() as u32,
                    image::ColorType::Rgba8,
                )
                .unwrap();
                eprintln!("Image saved to {path:?}.");
            }
        }
    }
}
