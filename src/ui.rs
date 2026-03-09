use eframe::egui::{self, Align2, Color32, FontId, RichText, Stroke, Vec2};

use crate::config::Section;

// ── Palette ─────────────────────────────────────────────────────────────────

// Dracula
const BG: Color32          = Color32::from_rgb(18, 18, 24);       // Near black
const HEADER_BG: Color32   = Color32::from_rgb(12, 12, 16);       // Darker
const SECTION_COLOR: Color32  = Color32::from_rgb(255, 121, 198); // Pink
const CATEGORY_COLOR: Color32 = Color32::from_rgb(139, 233, 253); // Cyan
const KEY_COLOR: Color32   = Color32::from_rgb(255, 255, 255);     // White
const KEY_BG: Color32      = Color32::from_rgb(68, 108, 138);     // Pastel steel blue
const KEY_BORDER: Color32  = Color32::from_rgb(88, 128, 158);     // Lighter steel
const DESC_COLOR: Color32  = Color32::from_rgb(248, 248, 242);    // Foreground
const RULE_COLOR: Color32  = Color32::from_rgb(68, 71, 90);       // Current Line
const HINT_COLOR: Color32  = Color32::from_rgb(98, 114, 164);     // Comment
const CONTENT_MAX_WIDTH: f32 = 1560.0;
const CONTENT_SIDE_PADDING: f32 = 12.0;
const LINE_SCROLL_STEP: f32 = 36.0;
const PAGE_SCROLL_FACTOR: f32 = 0.85;

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
            let keyboard_scroll = keyboard_scroll_delta(ctx, ui.available_height());
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if keyboard_scroll != 0.0 {
                        ui.scroll_with_delta(Vec2::new(0.0, keyboard_scroll));
                        ctx.request_repaint();
                    }
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let max_content_width =
                            (available - CONTENT_SIDE_PADDING * 2.0).min(CONTENT_MAX_WIDTH);
                        let side_padding =
                            ((available - max_content_width) * 0.5).max(CONTENT_SIDE_PADDING);
                        let content_width = (available - side_padding * 2.0).max(0.0);

                        ui.add_space(side_padding);
                        ui.allocate_ui_with_layout(
                            Vec2::new(content_width, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                for (i, section) in sections.iter().enumerate() {
                                    if i > 0 {
                                        ui.add_space(4.0);
                                        draw_rule(ui, RULE_COLOR);
                                        ui.add_space(4.0);
                                    }
                                    render_section(ui, section);
                                }
                            },
                        );
                        ui.add_space(side_padding);
                    });
                    ui.add_space(10.0);
                });
            draw_side_borders(ui, RULE_COLOR);
        });
}

fn keyboard_scroll_delta(ctx: &egui::Context, available_height: f32) -> f32 {
    let page_step = (available_height * PAGE_SCROLL_FACTOR).max(LINE_SCROLL_STEP * 4.0);
    ctx.input(|i| {
        let mut delta = 0.0;
        if i.key_down(egui::Key::ArrowDown) {
            delta += LINE_SCROLL_STEP;
        }
        if i.key_down(egui::Key::ArrowUp) {
            delta -= LINE_SCROLL_STEP;
        }
        if i.key_down(egui::Key::PageDown) {
            delta += page_step;
        }
        if i.key_down(egui::Key::PageUp) {
            delta -= page_step;
        }
        delta
    })
}

fn draw_side_borders(ui: &mut egui::Ui, color: Color32) {
    let rect = ui.max_rect();
    let stroke = Stroke::new(1.0, color);
    ui.painter()
        .line_segment([rect.left_top(), rect.left_bottom()], stroke);
    ui.painter()
        .line_segment([rect.right_top(), rect.right_bottom()], stroke);
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
        FontId::proportional(12.0),
        Color32::from_rgb(200, 205, 225),
    );
    ui.painter().text(
        rect.right_center() - Vec2::new(14.0, 0.0),
        Align2::RIGHT_CENTER,
        "Esc to close",
        FontId::proportional(9.5),
        HINT_COLOR,
    );
}

fn render_section(ui: &mut egui::Ui, section: &Section) {
    ui.label(
        RichText::new(&section.name)
            .font(FontId::proportional(19.0))
            .color(SECTION_COLOR)
            .strong(),
    );
    ui.add_space(4.0);

    const NCOLS: usize = 4;
    let cats = &section.categories;
    let col_width = (ui.available_width() - 4.0) / NCOLS as f32;
    let key_col_width = section_key_column_width(ui, section, col_width);

    egui::Grid::new(section.name.as_str())
        .num_columns(NCOLS)
        .min_col_width(col_width)
        .max_col_width(col_width)
        .spacing([2.0, 0.0])
        .show(ui, |ui| {
            for (i, cat) in cats.iter().enumerate() {
                ui.vertical(|ui| {
                    render_category(ui, cat, key_col_width);
                });
                if (i + 1) % NCOLS == 0 {
                    ui.end_row();
                }
            }
            if cats.len() % NCOLS != 0 {
                ui.end_row();
            }
        });
}

fn render_category(ui: &mut egui::Ui, category: &crate::config::Category, key_col_width: f32) {
    ui.add_space(4.0);
    ui.label(
        RichText::new(&category.name)
            .font(FontId::proportional(14.0))
            .color(CATEGORY_COLOR)
            .strong(),
    );
    draw_rule(ui, RULE_COLOR);
    ui.add_space(1.0);
    for entry in &category.entries {
        render_entry(ui, &entry.key, &entry.description, key_col_width);
    }
    ui.add_space(4.0);
}

fn render_entry(ui: &mut egui::Ui, key: &str, desc: &str, key_col: f32) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(6.0, 2.0);
        ui.allocate_ui_with_layout(
            Vec2::new(key_col, 20.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| key_badge(ui, key),
        );
        ui.label(
            RichText::new(desc)
                .font(FontId::proportional(14.0))
                .color(DESC_COLOR),
        );
    });
}

fn section_key_column_width(ui: &egui::Ui, section: &Section, col_width: f32) -> f32 {
    let mut widest = 0.0_f32;
    let font = FontId::monospace(13.0);
    for category in &section.categories {
        for entry in &category.entries {
            let galley = ui
                .painter()
                .layout_no_wrap(entry.key.clone(), font.clone(), KEY_COLOR);
            widest = widest.max(galley.size().x + 10.0); // 5px pad on each side
        }
    }

    let min_width = 56.0;
    let max_width = (col_width * 0.45).max(min_width);
    widest.clamp(min_width, max_width)
}

fn key_badge(ui: &mut egui::Ui, key: &str) {
    let text = egui::WidgetText::from(
        RichText::new(key)
            .font(FontId::monospace(13.0))
            .color(KEY_COLOR),
    );
    let galley = text.into_galley(
        ui,
        Some(egui::TextWrapMode::Extend),
        f32::INFINITY,
        FontId::monospace(13.0),
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
