use std::sync::Mutex;
use eframe::emath::Pos2;
use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints};
use crate::{BATCH_SIZE, LAYER1_PERFORMANCE, LAYER2_PERFORMANCE, NUM_LEVEL1_PROCESSORS, NUM_LEVEL2_PROCESSORS, NUM_PRODUCERS, PRODUCER_PERFORMANCE};

pub static PRODUCER_HISTORY: Mutex<Vec<Vec<f32>>> = Mutex::new(Vec::new());
pub static LAYER1_HISTORY: Mutex<Vec<Vec<f32>>> = Mutex::new(Vec::new());
pub static LAYER2_HISTORY: Mutex<Vec<Vec<f32>>> = Mutex::new(Vec::new());

pub struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::Window::new("Producer Queue")
            .title_bar(false)
            .fixed_pos(Pos2::new(230.0, 10.0))
            .show(ctx, |ui| {
                let percent = crate::PRODUCER_PERCENT.load(std::sync::atomic::Ordering::Relaxed);
                let plot = Plot::new("Producer Queue")
                    .height(700.0)
                    .width(50.0);
                plot.show(ui, |plot_ui| {
                    let bars = vec![
                        Bar::new(1.0, percent as f64),
                    ];
                    let bars = BarChart::new(bars);
                    plot_ui.bar_chart(bars)
                });
        });

        egui::Window::new("Processor Queue")
            .title_bar(false)
            .fixed_pos(Pos2::new(530.0, 10.0))
            .show(ctx, |ui| {
                let percent = crate::LAYER1_PERCENT.load(std::sync::atomic::Ordering::Relaxed);
                let plot = Plot::new("Proc Queue")
                    .height(700.0)
                    .width(50.0);
                plot.show(ui, |plot_ui| {
                    let bars = vec![
                        Bar::new(1.0, percent as f64),
                    ];
                    let bars = BarChart::new(bars);
                    plot_ui.bar_chart(bars)
                });
            });

        let mut graph_lock = PRODUCER_HISTORY.lock().unwrap();
        for i in 0..NUM_PRODUCERS {
            let produced = PRODUCER_PERFORMANCE[i].load(std::sync::atomic::Ordering::Relaxed);
            graph_lock[i].push(produced as f32);
            if graph_lock[i].len() > 300 {
                graph_lock[i].remove(0);
            }

            let title = format!("Producer #{i}");
            let graph_title = format!("LG_Producer#{}", i);
            let pos = Pos2::new(10.0, i as f32 * 120.0);
            egui::Window::new(&title).fixed_pos(pos).show(ctx, |ui| {
                let display = PRODUCER_PERFORMANCE[i].load(std::sync::atomic::Ordering::Relaxed);
                let display = format!("Messages per second: {}", display);
                ui.label(&display);
                let plot = Plot::new(&graph_title)
                    .height(75.0)
                    .width(200.0);
                plot.show(ui, |plot_ui| {
                    let points: Vec<[f64; 2]> = graph_lock[i].iter().enumerate().map(|(x, y)| [x as f64, *y as f64]).collect();
                    let line = Line::new(PlotPoints::new(points));
                    plot_ui.line(line);
                });
            });
        }
        drop(graph_lock);

        let mut graph_lock = LAYER1_HISTORY.lock().unwrap();
        for i in 0..NUM_LEVEL1_PROCESSORS {
            let processed = LAYER1_PERFORMANCE[i].load(std::sync::atomic::Ordering::Relaxed);
            graph_lock[i].push(processed as f32);
            if graph_lock[i].len() > 300 {
                graph_lock[i].remove(0);
            }

            let title = format!("Batch Combiner #{i}");
            let graph_title = format!("L1_Layer 1 Processor #{i}");
            let pos = Pos2::new(300.0, i as f32 * 120.0);
            egui::Window::new(&title).fixed_pos(pos).show(ctx, |ui| {
                let display = LAYER1_PERFORMANCE[i].load(std::sync::atomic::Ordering::Relaxed);
                let display = format!("Messages per second: {}", display);
                ui.label(&display);
                let plot = Plot::new(&graph_title)
                    .height(75.0)
                    .width(200.0);
                plot.show(ui, |plot_ui| {
                    let points: Vec<[f64; 2]> = graph_lock[i].iter().enumerate().map(|(x, y)| [x as f64, *y as f64]).collect();
                    let line = Line::new(PlotPoints::new(points));
                    plot_ui.line(line);
                });
            });
        }
        drop(graph_lock);

        let mut graph_lock = LAYER2_HISTORY.lock().unwrap();
        for i in 0..NUM_LEVEL2_PROCESSORS {
            let processed = LAYER2_PERFORMANCE[i].load(std::sync::atomic::Ordering::Relaxed);
            graph_lock[i].push(processed as f32);
            if graph_lock[i].len() > 300 {
                graph_lock[i].remove(0);
            }

            let title = format!("Layer 2 Processor #{i}");
            let graph_title = format!("L2_Layer 2 Processor #{i}");
            let pos = Pos2::new(600.0, i as f32 * 100.0);
            egui::Window::new(&title).fixed_pos(pos).show(ctx, |ui| {
                let display = LAYER2_PERFORMANCE[i].load(std::sync::atomic::Ordering::Relaxed);
                let display = format!("Messages per second: {}", display);
                ui.label(&display);

                let plot = Plot::new(&graph_title)
                    .height(75.0)
                    .width(200.0);
                plot.show(ui, |plot_ui| {
                    let points: Vec<[f64; 2]> = graph_lock[i].iter().enumerate().map(|(x, y)| [x as f64, *y as f64]).collect();
                    let line = Line::new(PlotPoints::new(points));
                    plot_ui.line(line);
                });
            });
        }
        drop(graph_lock);

        egui::Window::new("Batch Size")
            .title_bar(false)
            .fixed_pos(Pos2::new(1000.0, 1000.0))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    let batch_size = BATCH_SIZE.load(std::sync::atomic::Ordering::Relaxed);
                    ui.label(format!("Batch Size: {}", batch_size));
                    if ui.button("+").clicked() {
                        BATCH_SIZE.store(batch_size + 32, std::sync::atomic::Ordering::Relaxed);
                        ctx.request_repaint();
                    }
                    if ui.button("-").clicked() && batch_size > 32 {
                        BATCH_SIZE.store(batch_size - 32, std::sync::atomic::Ordering::Relaxed);
                        ctx.request_repaint();
                    }
                    let delay = crate::PROCESSING_DELAY_10TH_SECONDS.load(std::sync::atomic::Ordering::Relaxed);
                    ui.label(format!("Processing Delay: {:.1} seconds", delay as f32 / 10.0));
                    if ui.button("+").clicked() {
                        crate::PROCESSING_DELAY_10TH_SECONDS.store(delay + 1, std::sync::atomic::Ordering::Relaxed);
                        ctx.request_repaint();
                    }
                    if ui.button("-").clicked() && delay > 0 {
                        crate::PROCESSING_DELAY_10TH_SECONDS.store(delay - 1, std::sync::atomic::Ordering::Relaxed);
                        ctx.request_repaint();
                    }
                });
            });
    }
}