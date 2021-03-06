use tcod::RootConsole;
use tcod::colors::*;
use tcod::console::{BackgroundFlag, Console, TextAlignment};
use tcod::input;

// Traits
pub trait DrawUI {
    fn draw(&self, root: &mut RootConsole, cursor: (i32, i32));
}
pub trait MouseUI {
    fn bbox(&self) -> BBox;
    fn id(&self) -> String;
    fn bbox_colliding(&self, loc: (i32, i32)) -> Option<String> {
        let (bbs, bbe) = self.bbox();
        if (loc.0 >= bbs.0 - 4 && loc.1 <= bbs.1) &&
            (loc.0 <= bbe.0 - 4 && loc.1 >= bbe.1)
        {
            Some(self.id().clone())
        } else {
            None
        }
    }
}

// Data structures
type BBox = ((i32, i32), (i32, i32));

// UI Elements
pub struct Button {
    pub bbox: BBox,
    pub text: String,
    pub id: String,
}
impl Button {
    pub fn new(name: &'static str,
               pos: (i32, i32),
               size: (i32, i32))
        -> Button {
        Button {
            bbox: calculate_bbox(pos, size),
            text: format!("{:1$}", name, size.0 as usize),
            id: name.replace(" ", "_").to_lowercase(),
        }
    }
}
impl MouseUI for Button {
    fn id(&self) -> String { self.id.clone() }
    fn bbox(&self) -> BBox { self.bbox }
}
impl DrawUI for Button {
    fn draw(&self, root: &mut RootConsole, cursor: (i32, i32)) {
        root.set_default_foreground(BLACK);
        if self.bbox_colliding(cursor).is_some() {
            root.set_default_background(WHITE);
        } else {
            root.set_default_background(Color::new(100, 100, 100));
        }
        root.print_ex((self.bbox.0).0,
                      (self.bbox.0).1,
                      BackgroundFlag::Set,
                      TextAlignment::Center,
                      self.text.clone());
        root.set_default_foreground(WHITE);
        root.set_default_background(BLACK);
    }
}

pub struct Textbox {
    pub value: String,
    pub placeholder: String,
    pub bbox: BBox,
    pub id: String,
}
impl Textbox {
    pub fn new(text: &'static str,
               pos: (i32, i32),
               size: (i32, i32))
        -> Self {
        Textbox {
            value: "".to_string(),
            placeholder: text.to_string(),
            bbox: (pos, size),
            id: text.replace(" ", "_").to_lowercase(),
        }
    }
    pub fn input(&mut self, input: &input::Key) {
        if input.printable.is_alphanumeric() {
            self.value += &input.printable.to_string().to_owned();
        } else if input.code == input::KeyCode::Backspace {
            self.value.pop();
        }
    }
}
impl MouseUI for Textbox {
    fn id(&self) -> String { self.id.clone() }
    fn bbox(&self) -> BBox { self.bbox }
}
impl DrawUI for Textbox {
    fn draw(&self, root: &mut RootConsole, cursor: (i32, i32)) {
        root.set_default_foreground(BLACK);
        if self.bbox_colliding(cursor).is_some() {
            root.set_default_background(WHITE);
        } else {
            root.set_default_background(Color::new(100, 100, 100));
        }
        root.print_ex((self.bbox.0).0,
                      (self.bbox.0).1,
                      BackgroundFlag::Set,
                      TextAlignment::Center,
                      if self.value.is_empty() {
                          self.placeholder.clone()
                      } else {
                          self.value.clone()
                      });
        root.set_default_foreground(WHITE);
        root.set_default_background(BLACK);
    }
}

pub struct Selection {
    bbox: BBox,
}

// Functions
fn calculate_bbox(pos: (i32, i32), size: (i32, i32)) -> BBox {
    (pos, (pos.0 + size.0, pos.1 + size.1))
}

// Aggregate UI Elements
pub struct Layout {
    pub buttons: Vec<Button>,
}

impl Layout {
    pub fn new(elements: Vec<&'static str>,
               pos: (i32, i32),
               button_size: (i32, i32),
               wrap_at: i32)
        -> Layout {
        Layout {
            buttons: elements.iter()
                             .enumerate()
                             .map(|(i, text)| {
                let raw_x = i as i32 * button_size.0;
                Button::new(text,
                            (pos.0 + (raw_x % wrap_at),
                             pos.1 +
                                 (raw_x / wrap_at *
                                      (button_size.1 + 1))),
                            button_size)
            })
                             .collect(),
        }
    }
}

impl DrawUI for Layout {
    fn draw(&self, root: &mut RootConsole, cursor: (i32, i32)) {
        for button in self.buttons.iter() {
            button.draw(root, cursor);
        }
    }
}

impl MouseUI for Layout {
    fn bbox(&self) -> BBox { ((0, 0), (0, 0)) }
    fn id(&self) -> String { "".to_string() }
    fn bbox_colliding(&self, cursor: (i32, i32)) -> Option<String> {
        for button in self.buttons.iter().rev() {
            if let Some(x) = button.bbox_colliding(cursor) {
                return Some(x);
            }
        }
        None
    }
}

#[macro_export]
macro_rules! menu_event {
    ( ($mouse:expr, $menu:expr) $( $name:expr => $e:block )+ ) => {
        if $mouse.lbutton_pressed {
            match $menu.bbox_colliding(($mouse.cx as i32,
                                        $mouse.cy as i32)) {
                Some(item) => {
                    match item.trim().as_ref() {
                        $( $name => $e )+
                            _ => {}
                    }
                }
                None => {}
            }
        }
    }
}
