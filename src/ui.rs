use eframe::egui::{self, Align2, Color32, FontId, RichText, Stroke, Vec2};

use crate::config::Section;

// ── Palette ─────────────────────────────────────────────────────────────────

// Dracula
const BG: Color32          = Color32::from_rgb(18, 18, 24);       // Near black
const HEADER_BG: Color32   = Color32::from_rgb(12, 12, 16);       // Darker
const SECTION_COLOR: Color32  = Color32::from_rgb(255, 121, 198); // Pink
const CATEGORY_COLOR: Color32 = Color32::from_rgb(139, 233, 253); // Cyan
const KEY_COLOR: Color32   = Color32::from_rgb(80, 250, 123);     // Green
const KEY_BG: Color32      = Color32::from_rgb(68, 71, 90);       // Current Line
const KEY_BORDER: Color32  = Color32::from_rgb(98, 114, 164);     // Comment
const DESC_COLOR: Color32  = Color32::from_rgb(248, 248, 242);    // Foreground
const RULE_COLOR: Color32  = Color32::from_rgb(68, 71, 90);       // Current Line
const HINT_COLOR: Color32  = Color32::from_rgb(98, 114, 164);     // Comment

// ── App ─────────────────────────────────────────────────────────────────────

pub struct KeyViewerApp {
    sections: Vec<Section>,
    first_frame: bool,
}

impl KeyViewerApp {
    pub fn new(sections: Vec<Section>) -> Self {
        Self { sections, first_frame: true }
    }
}

impl eframe::App for KeyViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.first_frame {
            self.first_frame = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        render(ctx, &self.sections);
    }
}

// ── Rendering ────────────────────────────────────────────────────────────────

fn render(ctx: &egui::Context, sections: &[Section]) {
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(BG).inner_margin(egui::Margin::same(6)))
        .show(ctx, |ui| {
            render_header(ui);
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add_space(6.0);
                    for (i, section) in sections.iter().enumerate() {
                        if i > 0 {
                            ui.add_space(4.0);
                            draw_rule(ui, RULE_COLOR);
                            ui.add_space(4.0);
                        }
                        render_section(ui, section);
                    }
                    ui.add_space(10.0);
                });
        });
}

fn render_header(ui: &mut egui::Ui) {
    let width = ui.available_width();
    let height = 28.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, HEADER_BG);
    ui.painter().line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(1.0, RULE_COLOR),
    );
    ui.painter().text(
        rect.left_center() + Vec2::new(14.0, 0.0),
        Align2::LEFT_CENTER,
        "KeyViewer",
        FontId::proportional(13.0),
        Color32::from_rgb(200, 205, 225),
    );
    ui.painter().text(
        rect.right_center() - Vec2::new(14.0, 0.0),
        Align2::RIGHT_CENTER,
        "Esc to close",
        FontId::proportional(10.5),
        HINT_COLOR,
    );
}

fn render_section(ui: &mut egui::Ui, section: &Section) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.label(
            RichText::new(&section.name)
                .font(FontId::proportional(17.0))
                .color(SECTION_COLOR)
                .strong(),
        );
    });
    ui.add_space(4.0);

    const NCOLS: usize = 4;
    let padding = 6.0;
    let gap = 4.0;
    let col_width =
        (ui.available_width() - padding * 2.0 - gap * (NCOLS - 1) as f32) / NCOLS as f32;

    let cats = &section.categories;
    let rows = (cats.len() + NCOLS - 1) / NCOLS;

    for row in 0..rows {
        ui.horizontal_top(|ui| {
            ui.add_space(padding);
            for col in 0..NCOLS {
                let idx = row * NCOLS + col;
                if idx >= cats.len() { break; }
                ui.allocate_ui_with_layout(
                    Vec2::new(col_width, 0.0),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| render_category(ui, &cats[idx]),
                );
                if col < NCOLS - 1 { ui.add_space(gap); }
            }
        });
    }
}

fn render_category(ui: &mut egui::Ui, category: &crate::config::Category) {
    ui.add_space(4.0);
    ui.label(
        RichText::new(&category.name)
            .font(FontId::proportional(12.0))
            .color(CATEGORY_COLOR)
            .strong(),
    );
    draw_rule(ui, RULE_COLOR);
    ui.add_space(1.0);
    for entry in &category.entries {
        render_entry(ui, &entry.key, &entry.description);
    }
    ui.add_space(4.0);
}

fn render_entry(ui: &mut egui::Ui, key: &str, desc: &str) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(6.0, 2.0);
        let key_col = 90.0;
        ui.allocate_ui_with_layout(
            Vec2::new(key_col, 18.0),
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| key_badge(ui, key),
        );
        ui.label(
            RichText::new(desc)
                .font(FontId::proportional(12.0))
                .color(DESC_COLOR),
        );
    });
}

fn key_badge(ui: &mut egui::Ui, key: &str) {
    let text = egui::WidgetText::from(
        RichText::new(key)
            .font(FontId::monospace(11.0))
            .color(KEY_COLOR),
    );
    let galley = text.into_galley(
        ui,
        Some(egui::TextWrapMode::Extend),
        f32::INFINITY,
        FontId::monospace(11.0),
    );
    let pad = Vec2::new(5.0, 2.0);
    let size = galley.size() + pad * 2.0;
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    ui.painter().rect(rect, 4.0, KEY_BG, Stroke::new(1.0, KEY_BORDER), egui::StrokeKind::Middle);
    ui.painter().galley(rect.min + pad, galley, KEY_COLOR);
}

fn draw_rule(ui: &mut egui::Ui, color: Color32) {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), egui::Sense::hover());
    ui.painter()
        .line_segment([rect.left_center(), rect.right_center()], Stroke::new(1.0, color));
}
