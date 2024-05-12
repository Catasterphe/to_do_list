#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::{self, Read, Write}};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 700.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Aster's Tasklist",
        options,
        Box::new(|_cc| {
            let tasks = MyApp::load_tasks().unwrap_or_else(|_| Vec::new());

            let app = MyApp {
                tasks: tasks.clone(),
                ..Default::default()
            };
            Box::new(app)
        }),
    )
}

struct MyApp {
    new_task_name: String,
    tasks: Vec<Task>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            new_task_name: String::new(),
            tasks: Vec::new(),
        }
    }
}

impl MyApp {
    fn load_tasks() -> io::Result<Vec<Task>> {
        let mut file = File::open("tasks.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let tasks: Vec<Task> = serde_json::from_str(&contents)?;
        Ok(tasks)
    }

    fn save_tasks(&mut self) -> io::Result<()> {
        self.tasks.retain(|task| !task.completed);
        let serialized_tasks = serde_json::to_string(&self.tasks).unwrap();

        let mut file = File::create("tasks.json").expect("Failed to create/find tasks.json");
        file.write(serialized_tasks.as_bytes()).expect("Failed to write tasks.json");
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    name: String,
    completed: bool,
}

impl Task {
    fn new(name: impl Into<String>) -> Self {
        Task {
            name: name.into(),
            completed: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Err(err) = self.save_tasks() {
            eprintln!("Failed to save tasks: {}", err);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!(
                "Aster's Tasklist - {} task(s)",
                self.tasks.len()
            ));
            ui.horizontal(|ui| {
                let new_task_label = ui.label("Task Name:");
                ui.text_edit_singleline(&mut self.new_task_name)
                    .labelled_by(new_task_label.id);
                if ui.button("New Task").clicked() && !self.new_task_name.is_empty() {
                    self.tasks.push(Task::new(self.new_task_name.clone()));
                    self.new_task_name.clear()
                }
            });
            for task in &mut self.tasks {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut task.completed, "");
                    ui.label(format!("{}", &task.name));
                });
            }
        });
    }
}