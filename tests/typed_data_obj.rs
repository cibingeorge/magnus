use magnus::{
    class, define_class, embed::init, eval, function, gc, method, prelude::*, typed_data::Obj,
    value::Opaque, DataTypeFunctions, TypedData,
};

#[magnus::wrap(class = "Point", free_immediatly)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(TypedData)]
#[magnus(class = "Line", free_immediatly, mark)]
struct Line {
    #[magnus(opaque_attr_reader)]
    start: Opaque<Obj<Point>>,
    #[magnus(opaque_attr_reader)]
    end: Opaque<Obj<Point>>,
}

impl Line {
    fn new(start: Obj<Point>, end: Obj<Point>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    fn length(&self) -> f64 {
        let start = self.start();
        let end = self.end();

        (((end.x - start.x).pow(2) + (end.y - start.y).pow(2)) as f64).sqrt()
    }
}

impl DataTypeFunctions for Line {
    fn mark(&self) {
        gc::mark(self.start());
        gc::mark(self.end());
    }
}

#[test]
fn it_can_nest_wrapped_structs() {
    let _cleanup = unsafe { init() };

    let class = define_class("Point", class::object()).unwrap();
    class
        .define_singleton_method("new", function!(Point::new, 2))
        .unwrap();

    let class = define_class("Line", class::object()).unwrap();
    class
        .define_singleton_method("new", function!(Line::new, 2))
        .unwrap();
    class
        .define_method("length", method!(Line::length, 0))
        .unwrap();

    let result: f64 = eval(
        r#"
        start = Point.new(0, 0)
        finish = Point.new(10, 10)
        line = Line.new(start, finish)
        line.length
    "#,
    )
    .unwrap();

    assert!(result - 14.14213 < 0.00001);
}
