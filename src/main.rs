use eframe::egui::*;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native("SciGui", options, Box::new(|cc| Box::new(MyApp::new(cc))))
}

#[derive(Default)]
struct MyApp {
    plot_data: Vec<[f64; 2]>,
    delimiter: String,
    did_load_succeed: bool,
}

use std::error::Error;
use std::fs;
use std::path::PathBuf;
fn load_file_to_array(delimiter: &str, file: &PathBuf) -> Result<Vec<[f64; 2]>, Box<dyn Error>> {
    let contents = fs::read_to_string(file)?;
    let mut result: Vec<[f64; 2]> = Vec::new();

    if contents.is_empty() {
        Err("Empty File contents")?;
    }

    for s in contents.lines() {
        if let Some((x, y)) = s.split_once(delimiter) {
            result.push([x.trim().parse()?, y.trim().parse()?]);
        } else {
            eprintln!("Could not split Data {s}. Wrong format or delimieter ?");
        }
    }
    if result.is_empty() {
        Err("Empty results Vector")?;
    }
    Ok(result)
}

const DEFAULT_DATA: [[f64; 2]; 3] = [[0.0, 1.0], [2.0, 3.0], [3.0, 2.0]];

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            plot_data: DEFAULT_DATA.to_vec(),
            delimiter: ",".to_string(),
            did_load_succeed: false,
        }
    }
    fn display_side_panel(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        ui.heading("SciGui 📡");
        ui.horizontal(|ui| {
                if ui.button("Load Data").clicked() {
                    // TODO: spawn new dialog, when data failed to load
                    loop{
                    if let Some(file) = rfd::FileDialog::new()
                        .add_filter("text-file", &["txt"])
                        .add_filter("CSV-file", &["csv", "CSV"])
                        .set_directory(".")
                        .pick_file(){
                            // self.plot_data = load_file_to_array(&self.delimiter, &file)
                            //     .unwrap_or(DEFAULT_DATA.to_vec());
                            // break;
                            match load_file_to_array(&self.delimiter, &file){
                                Ok(data) =>{
                                    self.plot_data= data;
                                    self.did_load_succeed = true;
                                    break;
                                },
                                Err(e) =>{
                                    eprintln!("Error during File loading: {e}");
                                    if e.to_string() == "Empty results Vector" {
                                        self.plot_data = DEFAULT_DATA.to_vec();
                                        break;
                                    }
                                    continue;
                                },
                            }
                        } else {
                            self.did_load_succeed = false;
                            break;
                        }
                    }
                }
                // display load success
                if self.did_load_succeed {
                    ui.label("✅").on_hover_text("Shows, that the Data has been loaded sucessfully.");
                } else {
                    ui.label("❌").on_hover_text("Shows, that the Data has not been loaded sucessfully. Maybe you have picked the wrong Delimiter ?");
                }

                // choose Delimiter
                let delimiter_label = ui.label("Delimiter: ").on_hover_text(
                    "Specify the delimiter, which divides the x and y value of your Data.",
                );
                ui.text_edit_singleline(&mut self.delimiter)
                    .labelled_by(delimiter_label.id);
            });
        if ui.button("Save Plot").clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
        }
    }
    // Return position of the plot
    fn display_center_panel(&mut self, ctx: &egui::Context, ui: &mut Ui) -> Option<Rect> {
        let my_plot = Plot::new("My Plot").legend(Legend::default());

        // let's create a dummy line in the plot
        let inner = my_plot.show(ui, |plot_ui| {
            plot_ui.line(Line::new(PlotPoints::from(self.plot_data.clone())).name("Curve"));
        });
        // Remember the position of the plot
        Some(inner.response.rect)
    }

    fn handle_screenshot(
        &mut self,
        ctx: &egui::Context,
        screenshot: &Arc<ColorImage>,
        plot_location: &Rect,
    ) {
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

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut plot_rect = None;

        egui::SidePanel::right("options").show(ctx, |ui| {
            self.display_side_panel(ctx, ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            plot_rect = self.display_center_panel(ctx, ui);
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
            self.handle_screenshot(&ctx, &screenshot, &plot_location);
        }
    }
}
