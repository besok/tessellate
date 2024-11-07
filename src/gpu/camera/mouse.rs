use winit::dpi::PhysicalPosition;

#[derive(Debug, Default)]
pub struct Mouse {
    lb_pressed: bool,
    rb_pressed: bool,
    last_pos: Option<PhysicalPosition<f64>>,
}

impl Mouse {
    pub fn is_pressed(&self) -> bool {
        self.lb_pressed || self.rb_pressed
    }

    pub fn is_left_pressed(&self) -> bool {
        self.lb_pressed
    }

    pub fn is_right_pressed(&self) -> bool {
        self.rb_pressed
    }

    pub fn set_pos(&mut self, pos: PhysicalPosition<f64>) {
        self.last_pos = Some(pos);
    }

    pub fn pos(&self) -> Option<PhysicalPosition<f64>> {
        self.last_pos
    }

    pub fn clean_pos(&mut self) {
        self.last_pos = None;
    }

    pub fn set_left_button(&mut self, pressed: bool) -> bool{
        self.lb_pressed = pressed;
        if !pressed {
            self.clean_pos();
        }
        true
    }

    pub fn set_right_button(&mut self, pressed: bool) -> bool {
        self.rb_pressed = pressed;
        if !pressed {
            self.clean_pos();
        }
        true
    }
}