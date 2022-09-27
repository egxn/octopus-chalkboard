use eframe::{egui, emath::{self, RectTransform}, NativeOptions, CreationContext};
use egui::{Frame, Pos2, Stroke, Color32, Response, Sense, Rect, Shape, CentralPanel, Key};

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
  color: Color32,
  keyboard: Keyboard,
  lines: Vec<Vec<Pos2>>,
  stroke: Stroke,
  stroke_width: f32,
}

impl Default for Octopus {
  fn default() -> Self {
    Self {
      color: Color32::from_rgb(0, 0, 0),
      lines: Default::default(),
      keyboard: Keyboard::new(),
      stroke: Stroke::new(1.0, Color32::from_rgb(0, 0, 0)),
      stroke_width: 1.0,
    }
  }
}

struct Keyboard {
  colors: Vec<egui::Key>,
  stroke_width: Vec<egui::Key>,
}

impl Keyboard {
  fn new() -> Self {
    Self {
      colors: vec![
        Key::Num1,
        Key::Num2,
        Key::Num3,
        Key::Num4,
        Key::Num5,
        Key::Num6,
        Key::Num7,
        Key::Num8,
        Key::Num9,
      ],
      stroke_width: vec![
        Key::Q,
        Key::W,
      ],
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
    custom_window_frame(ctx, frame, "Octopus 🐙", |ui| {
      let events = ui.input().events.clone();
      for event in &events {
        match event {
          egui::Event::Key{key, pressed, modifiers} => {
            if self.keyboard.colors.contains(key) && *pressed {
              self.color = change_color(key);
              self.stroke = Stroke::new(self.stroke_width, self.color);
            } else if self.keyboard.stroke_width.contains(key) && *pressed {
              self.stroke_width = change_stroke_width(key, &self.stroke_width);
              self.stroke = Stroke::new(self.stroke_width, self.color);
            } else if key.eq(&Key::Q) && modifiers.ctrl {
              println!("CTRL + Q");
            }
          },
          _ => {}
        }
      }
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

fn change_color(key: &Key) -> Color32 {
  match key {
    Key::Num0 => Color32::from_rgb(234, 118, 203),
    Key::Num1 => Color32::from_rgb(136, 57, 239),
    Key::Num2 => Color32::from_rgb(210, 15, 57),
    Key::Num3 => Color32::from_rgb(254, 100, 11),
    Key::Num4 => Color32::from_rgb(64, 160, 43),
    Key::Num5 => Color32::from_rgb(32, 159, 181),
    Key::Num6 => Color32::from_rgb(30, 102, 245),
    Key::Num7 => Color32::from_rgb(114, 135, 253),
    Key::Num8 => Color32::from_rgb(76, 79, 105),
    Key::Num9 => Color32::from_rgb(239, 241, 245),
      _ => Color32::from_rgb(202, 158, 230),
  }
}

fn change_stroke_width(key: &Key, width: &f32) -> f32 {
  match key {
    Key::Q => width + 1.0,
    Key::W => width - 1.0,
    _ => width.clone(),
  }
}