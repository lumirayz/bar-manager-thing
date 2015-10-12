use ::component::core::Component;
use ::bar::{Bar, Alignment};

pub struct BgColor {
    color: String,
    child: Box<Component>
}

pub struct FgColor {
    color: String,
    child: Box<Component>
}

pub struct UColor {
    color: String,
    child: Box<Component>
}

pub struct Font {
    id: u64,
    child: Box<Component>
}

pub struct Align {
    align: Alignment,
    child: Box<Component>
}

impl Component for BgColor {
    fn render(&self, bar: &mut Bar) {
        bar.bgcol(&self.color);
        self.child.render(bar);
        bar.bgcol("-");
    }
}

impl Component for FgColor {
    fn render(&self, bar: &mut Bar) {
        bar.fgcol(&self.color);
        self.child.render(bar);
        bar.fgcol("-");
    }
}

impl Component for UColor {
    fn render(&self, bar: &mut Bar) {
        bar.ucol(&self.color);
        self.child.render(bar);
        bar.ucol("-");
    }
}

impl Component for Font {
    fn render(&self, bar: &mut Bar) {
        bar.font(self.id);
        self.child.render(bar);
    }
}

impl Component for Align {
    fn render(&self, bar: &mut Bar) {
        bar.align(self.align.clone());
        self.child.render(bar);
    }
}

pub fn bg<T: ToString>(color: T, child: Box<Component>) -> BgColor {
    BgColor {color: color.to_string(), child: child}
}

pub fn fg<T: ToString>(color: T, child: Box<Component>) -> FgColor {
    FgColor {color: color.to_string(), child: child}
}

pub fn u<T: ToString>(color: T, child: Box<Component>) -> UColor {
    UColor {color: color.to_string(), child: child}
}

pub fn font(id: u64, child: Box<Component>) -> Font {
    Font {id: id, child: child}
}

pub fn align_left(child: Box<Component>) -> Align {
    Align {align: Alignment::Left, child: child}
}
pub fn align_center(child: Box<Component>) -> Align {
    Align {align: Alignment::Center, child: child}
}
pub fn align_right(child: Box<Component>) -> Align {
    Align {align: Alignment::Right, child: child}
}
