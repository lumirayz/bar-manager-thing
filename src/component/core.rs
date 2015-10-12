use ::bar::Bar;

macro_rules! component {
    [ $( $x:expr ), * ] => ( {
        let mut vec: Vec<Box<Component>> = Vec::new();
        $(
            vec.push(Box::new($x));
        )*
        ConcatComponent::new(vec)
    } );
    ( $f:ident => [ $( $x:expr ), * ] ) => ( {
        $f(Box::new(component![ $( $x ), * ]))
    } );
    ( $f:ident( $( $a:expr), * ) => [ $( $x:expr ), * ] ) => ( {
        $f($($a)*, Box::new(component![ $( $x ), * ]))
    } );
}

macro_rules! lazy {
    ($e:expr) => { {
        let c : Box<Fn() -> Box<Component>> = Box::new(|| Box::new($e));
        c
    } }
}

macro_rules! dynamic {
    ( init => $init:expr, render ( $data:ident : $typ:ty ) => $render:expr ) => { {
        struct S { inner: $typ }
        impl Component for S {
            fn render(&self, bar : &mut Bar) {
                let $data = &self.inner;
                $render.render(bar);
            }
        }
        S { inner: $init }
    } }
}

pub trait Component {
    fn render(&self, &mut Bar);
}

impl Component for Box<Fn() -> Box<Component>> {
    fn render(&self, bar: &mut Bar) {
        let c: Box<Component> = self();
        c.render(bar);
    }
}

impl<T: ToString> Component for T {
    fn render(&self, bar: &mut Bar) {
        bar.text(&(self.to_string()));
    }
}

pub struct ConcatComponent {
    children: Vec<Box<Component>>
}

impl ConcatComponent {
    pub fn new(children: Vec<Box<Component>>) -> ConcatComponent {
        ConcatComponent {children: children}
    }
}

impl Component for ConcatComponent {
    fn render(&self, bar: &mut Bar) {
        for ref c in &self.children {
            c.render(bar);
        }
    }
}
