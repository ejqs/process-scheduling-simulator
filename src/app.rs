use std::hash::{Hash, Hasher};

use crate::process_scheduler::{job_builder, *};
use egui_dropdown::DropDownBox;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    job_count: u32,
    jobs: Vec<Job>,
    // TODO: Understand serialization
    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    process_scheduling_algorithms: Vec<String>,
    time_quantum: u32,
    buf: String,
    viewport_open: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            job_count: 1,
            jobs: Vec::new(),
            value: 2.7,
            process_scheduling_algorithms: vec![
                "Random".into(),
                "First Come First Serve (FCFS)".into(),
                "Shortest Job Next (SJN)".into(),
                "Shortest Remaining Time (SRT)".into(),
                "Round Robin".into(),
            ],
            time_quantum: 0,
            buf: String::new(),
            viewport_open: false,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // Code Below Enables Persistence
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Process Scheduling Simulator");

            ui.horizontal(|ui| {
                ui.label("Number of Jobs: ");
                ui.add(
                    egui::DragValue::new(&mut self.job_count)
                        .range(0..=u16::MAX)
                        .speed(0.02),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Process Scheduling Algorithm:");
                // ui.text_edit_singleline(&mut self.label);
                ui.add(
                    DropDownBox::from_iter(
                        &self.process_scheduling_algorithms,
                        "test_dropbox",
                        &mut self.buf,
                        |ui, text| ui.selectable_label(false, text),
                    )
                    // choose whether to filter the box items based on what is in the text edit already
                    // default is true when this is not used
                    .hint_text("Choose an algorithm")
                    .filter_by_input(false)
                    // choose whether to select all text in the text edit when it gets focused
                    // default is false when this is not used
                    .select_on_focus(true), // passes through the desired width to the text edit
                                            // default is None internally, so TextEdit does whatever its default implements
                );
            });

            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Button::new("Open Simulator")
                            .fill(egui::Color32::from_rgb(100, 149, 237)),
                    )
                    .clicked()
                {
                    if self.viewport_open == true {
                        self.viewport_open = false
                    } else {
                        self.viewport_open = true
                    };
                    println!("{:?}", self.buf);
                    println!("{}", self.job_count);
                }
                if ui
                    .add(
                        egui::Button::new("Randomize Details")
                            .fill(egui::Color32::from_rgb(100, 149, 237)),
                    )
                    .clicked()
                {
                    self.jobs = randomize_jobs(self.jobs.clone())
                }

                // TODO: Allow for User Closing
                if self.viewport_open {
                    spawn_new_window(
                        ctx,
                        self.buf.clone(),
                        self.jobs.clone(),
                        self.time_quantum.clone(),
                    );
                }
            });

            if self.jobs.len() as u32 != self.job_count {
                self.jobs = job_builder(&self.jobs, self.job_count);
            }
            ui.add_space(16.0);
            // TIME QUANTUM
            if self.buf == self.process_scheduling_algorithms[4] {
                ui.horizontal(|ui| {
                    ui.label(format!("Time Quantum: "));
                    ui.add(
                        egui::DragValue::new(&mut self.time_quantum)
                            .range(0..=u16::MAX)
                            .speed(0.02),
                    );
                });
            }

            ui.horizontal(|ui| {
                ui.label("Job Details | CPU Cycle | Arrival Time");
            });

            for job in &mut self.jobs {
                ui.horizontal(|ui| {
                    ui.label(format!("Jobs: {}", job.job_name));
                    ui.add(egui::DragValue::new(&mut job.needed_cpu_cycle).range(0..=u16::MAX));
                    ui.add(egui::DragValue::new(&mut job.arrival_time).range(0..=u16::MAX));
                });
            }
        });
    }
}

fn spawn_new_window(ctx: &egui::Context, algorithm: String, jobs: Vec<Job>, time_quantum: u32) {
    let ctx_clone = ctx.clone();

    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of(1),
        egui::ViewportBuilder::default(),
        move |_, _| {
            // Define the UI for the new viewport here
            egui::CentralPanel::default().show(&ctx_clone, |ui| {
                timeline_builder_screen(ui, algorithm.clone(), jobs.clone(), time_quantum.clone());
            });
        },
    );
}

// FIXME: Updates only on mouse hover on second window
fn timeline_builder_screen(
    ui: &mut egui::Ui,
    algorithm: String,
    jobs: Vec<Job>,
    time_quantum: u32,
) {
    // TODO: ALLOW TO ONLY RUN ONCE
    let (_scheduled_jobs, timeline) =
        process_scheduler(algorithm.clone(), jobs.clone(), time_quantum.clone());

    let mut job_segments = Vec::new();

    ui.horizontal(|ui| {
        for i in 0..timeline.len() {
            let (job_name, start_time, end_time) = &timeline[i];
            job_segments.push((job_name.clone(), *start_time as f32, *end_time as f32));
        }

        // println!("{:?}", job_segments);
        let painter = ui.painter();
        let total_time = if let Some(last) = timeline.last() {
            last.2 as f32
        } else {
            1.0 // Default value when timeline is empty
        };
        let width = ui.available_width();
        let height = 50.0;
        let rect = egui::Rect::from_min_size(ui.cursor().min, egui::vec2(width, height));
        painter.rect_filled(rect, 0.0, egui::Color32::LIGHT_GRAY);

        for (job_name, start_time, end_time) in job_segments {
            let color = {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                job_name.hash(&mut hasher);
                let hash = hasher.finish();
                let r = (hash & 0xFF) as u8;
                let g = ((hash >> 8) & 0xFF) as u8;
                let b = ((hash >> 16) & 0xFF) as u8;
                egui::Color32::from_rgb(r, g, b)
            };
            let x_start = rect.left() + (start_time / total_time) * rect.width();
            let x_end = rect.left() + (end_time / total_time) * rect.width();
            let job_rect = egui::Rect::from_min_max(
                egui::pos2(x_start, rect.top()),
                egui::pos2(x_end, rect.bottom()),
            );
            painter.rect_filled(job_rect, 0.0, color);
            painter.text(
                job_rect.center(),
                egui::Align2::CENTER_CENTER,
                job_name,
                egui::FontId::default(),
                egui::Color32::BLACK,
            );

            // Draw a line to separate the jobs
            painter.line_segment(
                [
                    egui::pos2(x_end, rect.top()),
                    egui::pos2(x_end, rect.bottom()),
                ],
                (0.5, egui::Color32::BLACK),
            );
        }
    });
    ui.add_space(50.0);
    ui.label(format!("{}", algorithm));
    ui.label(format!("{:?}", timeline));
    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        powered_by_egui_and_eframe(ui);
        egui::warn_if_debug_build(ui);
    });
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
