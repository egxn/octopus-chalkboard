use eframe::{egui, emath::{self, RectTransform}, NativeOptions, CreationContext};
use egui::{Frame, Pos2, Stroke, Color32, Response, Sense, Rect, Shape, CentralPanel};

fn main() {
  let options: NativeOptions = eframe::NativeOptions {
    decorated: false,
    transparent: true,
    fullscreen: true,
    ..Default::default()
  };

  eframe::run_native(
    "Octopus 🐙",
    options,
    Box::new(|_cc: &CreationContext| Box::new(Octopus::default())),
  );
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
struct Octopus {
  lines: Vec<Vec<Pos2>>,
  stroke: Stroke,
}

impl Default for Octopus {
  fn default() -> Self {
    Self {
      lines: Default::default(),
      stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
    }
  }
}

impl Octopus {
  fn ui_content(&mut self ,ui: &mut egui::Ui) -> Response {
    let (mut response, painter) =
    ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());
  
    let to_screen: RectTransform = emath::RectTransform::from_to(
      Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
      response.rect,
    );
    let from_screen: RectTransform = to_screen.inverse();
  
    if self.lines.is_empty() {
      self.lines.push(vec![]);
    }
  
    let current_line = self.lines.last_mut().unwrap();
  
    if let Some(pointer_pos) = response.interact_pointer_pos() {
      let canvas_pos = from_screen * pointer_pos;
      if current_line.last() != Some(&canvas_pos) {
          current_line.push(canvas_pos);
          response.mark_changed();
      }
    } else if !current_line.is_empty() {
      self.lines.push(vec![]);
      response.mark_changed();
    }
  
    let mut shapes: Vec<Shape> = vec![];
    for line in &self.lines {
      if line.len() >= 2 {
          let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
          shapes.push(egui::Shape::line(points, self.stroke));
      }
    }
    painter.extend(shapes);
  
    response
  }
}

impl eframe::App for Octopus {
  fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
    egui::Rgba::TRANSPARENT
  }

  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    custom_window_frame(ctx, frame, "Octuopus 🐙", |ui| {
      Frame::canvas(ui.style())
      .fill(egui::Color32::TRANSPARENT)
      .show(ui, |ui| {
        self.ui_content(ui);
      });
    });
  }
}

fn custom_window_frame(
  ctx: &egui::Context,
  _frame: &mut eframe::Frame,
  _title: &str,
  add_contents: impl FnOnce(&mut egui::Ui),
) {
  let text_color = ctx.style().visuals.text_color();

  CentralPanel::default()
    .frame(Frame::none())
    .show(ctx, |ui| {
        let rect = ui.max_rect();
        let painter = ui.painter();
        painter.rect(
          rect.shrink(1.0),
          2.0,
          egui::Rgba::TRANSPARENT,
          Stroke::new(1.0, text_color),
        );

        let content_rect = {
          let mut rect = rect;
          rect.min.y = 0.0;
          rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
      });
}
