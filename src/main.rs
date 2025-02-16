use eframe::egui::{self, ViewportCommand};

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_decorations(false)
        .with_inner_size([400.0, 400.0])
        .with_min_inner_size([400.0, 400.0])
        .with_transparent(true),
    ..Default::default()
    };
    eframe::run_native("Rustic Mp3", 
                       options, 
                       Box::new(|cc| Ok(Box::new(RustMp3App::new(cc)))));
}

#[derive(Default)]
struct RustMp3App {}

impl RustMp3App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for RustMp3App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, context: &egui::Context, frame: &mut eframe::Frame) {
        windowFrame(context, "RusticMp3", |ui| {
            ui.label("Hello World!");
        });
    }
}

fn windowFrame(context: &egui::Context, title: &str, addContents: impl FnOnce(&mut egui::Ui)) {
    use egui::{CentralPanel, UiBuilder};

    let panelFrame = egui::Frame::new()
        .fill(context.style().visuals.window_fill())
        .corner_radius(5)
        .stroke(context.style().visuals.widgets.noninteractive.fg_stroke)
        .inner_margin(5)
        .outer_margin(1);

    CentralPanel::default().frame(panelFrame).show(context, |ui| {
        let appRectangle = ui.max_rect();

        let titleBarHeight = 32.0;
        let titleBarRectangle = {
            let mut rectangle = appRectangle;
            rectangle.max.y = rectangle.min.y + titleBarHeight;
            rectangle
        };
        titleBarUi(ui, titleBarRectangle, title);

        let contentRectangle = {
            let mut rectangle = appRectangle;
            rectangle.min.y = titleBarRectangle.max.y;
            rectangle
        }
        .shrink(4.0);

        let mut contentUi = ui.new_child(UiBuilder::new().max_rect(contentRectangle));
        addContents(&mut contentUi);
    });
}

fn titleBarUi(ui: &mut egui::Ui, titleBarRectangle: eframe::epaint::Rect, title: &str) {
    use egui::{vec2, Align2, FontId, Id, PointerButton, Sense, UiBuilder};

    let painter = ui.painter();

    let titleBarResponse = ui.interact(titleBarRectangle,
                                       Id::new("title_bar"),
                                       Sense::click_and_drag());

    painter.text(titleBarRectangle.center(),
                 Align2::CENTER_CENTER,
                 title,
                 FontId::proportional(20.0),
                 ui.style().visuals.text_color());

    painter.line_segment([ titleBarRectangle.left_bottom() + vec2(1.0, 0.0),
                           titleBarRectangle.right_bottom() + vec2(-1.0, 0.0) ],
                         ui.visuals().widgets.noninteractive.bg_stroke);

    if titleBarResponse.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx()
        .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    }

    if titleBarResponse.drag_started_by(PointerButton::Primary) {
        ui.ctx()
        .send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.allocate_new_ui(
        UiBuilder::new()
            .max_rect(titleBarRectangle)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            closeMaximizeMinimize(ui);
        },
    );
}

fn closeMaximizeMinimize(ui: &mut egui::Ui) {
    use egui::{Button, RichText};

    let buttonHeight = 12.0;
    let closeResponse = ui
        .add(Button::new(RichText::new("❌").size(buttonHeight)))
        .on_hover_text("Close the window");
    if closeResponse.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    let isMaximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if isMaximized {
        let maximizedResponse = ui
            .add(Button::new(RichText::new("🗗").size(buttonHeight)))
            .on_hover_text("Restore window");
        if maximizedResponse.clicked() {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(false));
        }
    } else {
        let maximizedResponse = ui
            .add(Button::new(RichText::new("🗗").size(buttonHeight)))
            .on_hover_text("MaximizeWindow");
        if maximizedResponse.clicked() {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(true));
        }
    }

    let minimizedResponse = ui
        .add(Button::new(RichText::new("🗕").size(buttonHeight)))
        .on_hover_text("Minimize the window");
    if minimizedResponse.clicked() {
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}
