use crate::process_scheduler::{job_builder, *};
use egui::RichText;
use egui_dropdown::DropDownBox;
use std::hash::{Hash, Hasher};

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
    open_sim_string: String,
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
            time_quantum: 5,
            buf: "First Come First Serve (FCFS)".to_string(),
            viewport_open: false,
            open_sim_string: "Open Simulator".to_string(),
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

        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:

        //     egui::menu::bar(ui, |ui| {
        //         // NOTE: no File->Quit on web pages!
        //         let is_web = cfg!(target_arch = "wasm32");
        //         if !is_web {
        //             ui.menu_button("File", |ui| {
        //                 if ui.button("Quit").clicked() {
        //                     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        //                 }
        //             });
        //             ui.add_space(16.0);
        //         }

        //         egui::widgets::global_theme_preference_buttons(ui);
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Process Scheduling Simulator");

            ui.horizontal(|ui| {
                ui.label("Number of Jobs: ");
                ui.add(
                    egui::DragValue::new(&mut self.job_count)
                        .range(1..=u16::MAX)
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
                        egui::Button::new(self.open_sim_string.clone())
                            .fill(egui::Color32::from_rgb(100, 149, 237)),
                    )
                    .clicked()
                {
                    if self.viewport_open == true {
                        self.viewport_open = false;
                        self.open_sim_string = "Open Simulator".to_string();
                    } else {
                        self.viewport_open = true;
                        self.open_sim_string = "Close Simulator".to_string();
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
                // TODO: Force Close Window on Unsafe Operations
                if self.viewport_open {
                    let algorithm_num = match self.buf.as_str() {
                        "Random" => 0,
                        "First Come First Serve (FCFS)" => 1,
                        "Shortest Job Next (SJN)" => 2,
                        "Shortest Remaining Time (SRT)" => 3,
                        "Round Robin" => 4,
                        _ => -1, // Unknown, program will panic
                    };
                    if algorithm_num != -1 {
                        self.spawn_new_window(
                            ctx,
                            self.buf.clone(),
                            self.jobs.clone(),
                            self.time_quantum.clone(),
                        );
                    } else {
                        self.viewport_open = false;
                        self.buf = "First Come First Serve (FCFS)".to_string();
                        self.viewport_open = true;
                    }
                }
            });

            if self.jobs.len() as u32 != self.job_count {
                self.jobs = job_builder(&self.jobs, self.job_count);
            }
            ui.add_space(16.0);
            // TIME QUANTUM
            // TODO: MAKE SURE THIS IS CHECKED!
            if self.buf == self.process_scheduling_algorithms[4] {
                ui.horizontal(|ui| {
                    ui.label(format!("Time Quantum: "));
                    ui.add(
                        egui::DragValue::new(&mut self.time_quantum)
                            .range(1..=u16::MAX)
                            .speed(0.02),
                    );
                });
            }

            egui::Grid::new("some_unique_id")
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Job Details");
                    ui.label("CPU Cycle");
                    ui.label("Arrival Time");
                    ui.label("Move Up");
                    ui.label("Move Down");
                    ui.end_row();

                    for i in 0..self.jobs.len() {
                        let color = {
                            let mut hasher = std::collections::hash_map::DefaultHasher::new();
                            self.jobs[i].job_name.hash(&mut hasher);
                            let hash = hasher.finish();
                            let r = (hash & 0xFF) as u8;
                            let g = ((hash >> 8) & 0xFF) as u8;
                            let b = ((hash >> 16) & 0xFF) as u8;
                            egui::Color32::from_rgb(r, g, b)
                        };

                        ui.label(
                            RichText::new(format!("JOB {}", self.jobs[i].job_name))
                                .background_color(color)
                                .color(
                                    if (0.299 * color.r() as f32
                                        + 0.587 * color.g() as f32
                                        + 0.114 * color.b() as f32)
                                        > 128.0
                                    {
                                        egui::Color32::BLACK
                                    } else {
                                        egui::Color32::WHITE
                                    },
                                )
                                .strong(),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.jobs[i].needed_cpu_cycle)
                                .range(1..=u16::MAX),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.jobs[i].arrival_time)
                                .range(0..=u16::MAX),
                        );
                        // move up [a][b] swap with previous
                        if ui
                            .add(egui::Button::new("^").fill(if i == 0 {
                                egui::Color32::from_rgb(200, 200, 200)
                            } else {
                                egui::Color32::from_rgb(100, 149, 237)
                            }))
                            .clicked()
                        {
                            // Prevent action if job is first
                            if i != 0 {
                                let a = self.jobs[i - 1].clone();
                                let b = self.jobs[i].clone();

                                self.jobs[i] = a;
                                self.jobs[i - 1] = b;
                            }
                        }
                        // move up [b][a] swap with next
                        if ui
                            .add(egui::Button::new("v").fill(if i == self.jobs.len() - 1 {
                                egui::Color32::from_rgb(200, 200, 200)
                            } else {
                                egui::Color32::from_rgb(100, 149, 237)
                            }))
                            .clicked()
                        {
                            // Prevent action if job is last
                            if i != self.jobs.len() - 1 {
                                let a = self.jobs[i + 1].clone();
                                let b = self.jobs[i].clone();

                                self.jobs[i] = a;
                                self.jobs[i + 1] = b;
                            }
                        }
                        ui.end_row();
                    }
                });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                github_link(ui);
            });
        });
    }
}

impl App {
    fn spawn_new_window(
        &mut self,
        ctx: &egui::Context,
        algorithm: String,
        jobs: Vec<Job>,
        time_quantum: u32,
    ) {
        // return value adjusts "viewport_open"
        let ctx_clone = ctx.clone();
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of(1),
            egui::ViewportBuilder::default(),
            move |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );
                // Define the UI for the new viewport here
                egui::CentralPanel::default().show(&ctx_clone, |ui| {
                    timeline_builder_screen(
                        ui,
                        algorithm.clone(),
                        jobs.clone(),
                        time_quantum.clone(),
                    );
                });

                // I want to die. multiple days of trying to understand egui docmentation
                // the solution looks like this -> https://github.com/emilk/egui/discussions/5306#discussioncomment-11067057
                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    // println!("recieved close request!!");
                    self.viewport_open = false;
                };
            },
        );
    }
}
// FIXME: Updates only on mouse hover on second window
fn timeline_builder_screen(
    ui: &mut egui::Ui,
    algorithm: String,
    jobs: Vec<Job>,
    time_quantum: u32,
) {
    // TODO: ALLOW TO ONLY RUN ONCE
    let (mut returned_jobs, timeline) =
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

        for i in 0..job_segments.len() {
            // let mut previous_start_time: f32 = 0.0;
            let mut previous_end_time: f32 = 0.0;
            let next_start_time: f32;
            // let mut next_end_time: f32 = 0.0;

            let (job_name, start_time, end_time) = &job_segments[i];
            if i != 0 {
                (_, _, previous_end_time) = job_segments[i - 1];
            }
            if i + 1 != job_segments.len() {
                (_, next_start_time, _) = job_segments[i + 1];
            } else {
                next_start_time = -1.0;
            }

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

            // Paint Start
            if previous_end_time <= *start_time {
                painter.text(
                    job_rect.left_bottom() + egui::vec2(0.0, 20.0),
                    egui::Align2::LEFT_BOTTOM,
                    start_time,
                    egui::FontId::default(),
                    egui::Color32::BLACK,
                );
            }

            if *end_time < next_start_time || next_start_time == -1.0 {
                // Paint End
                painter.text(
                    job_rect.right_bottom() + egui::vec2(0.0, 20.0),
                    egui::Align2::RIGHT_BOTTOM,
                    end_time,
                    egui::FontId::default(),
                    egui::Color32::BLACK,
                );
            }

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
    ui.add_space(80.0);
    // ui.label(format!("",));
    ui.label(format!("{}", algorithm));
    ui.label(format!("{:?}", timeline));

    let mut total_turnaround_time: f64 = 0.0;

    for job in &mut returned_jobs {
        total_turnaround_time += job.turnaround_time as f64;
    }
    let average_turnaround_time = total_turnaround_time / returned_jobs.len() as f64;

    egui::Grid::new("some_unique_id")
        .striped(true)
        .show(ui, |ui| {
            ui.label("Average Turnaround Time: ");
            ui.label(format!("{:.2}", average_turnaround_time));
            ui.end_row();

            ui.label("Job Name");
            ui.label("Completion Time");
            ui.label("Turn Around");
            ui.end_row();
            for job in &mut returned_jobs {
                let color = {
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    job.job_name.hash(&mut hasher);
                    let hash = hasher.finish();
                    let r = (hash & 0xFF) as u8;
                    let g = ((hash >> 8) & 0xFF) as u8;
                    let b = ((hash >> 16) & 0xFF) as u8;
                    egui::Color32::from_rgb(r, g, b)
                };
                // ui.label(format!("Job {}", job.job_name));
                ui.label(
                    RichText::new(format!("JOB {}", job.job_name))
                        .background_color(color)
                        .color(
                            if (0.299 * color.r() as f32
                                + 0.587 * color.g() as f32
                                + 0.114 * color.b() as f32)
                                > 128.0
                            {
                                egui::Color32::BLACK
                            } else {
                                egui::Color32::WHITE
                            },
                        )
                        .strong(),
                );
                ui.label(format!("{}", job.completion_time));
                ui.label(format!("{}", job.turnaround_time));
                total_turnaround_time += job.turnaround_time as f64;
                ui.end_row();
            }
        });

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

fn github_link(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Github: ");
        ui.hyperlink_to(
            "ejqs/process-scheduling-simulator",
            "https://github.com/ejqs/process-scheduling-simulator",
        );
    });
}
