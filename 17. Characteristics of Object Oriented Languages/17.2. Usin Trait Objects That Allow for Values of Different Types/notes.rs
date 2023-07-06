// Definition of the Draw trait
pub trait Draw {
    fn draw(&self);
}

// Definition of the Screen struct with a components field holding a vector of trait objects that implement the Draw trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// A run method on Screen that calls the draw method on each component
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// An alternate implementation of the Screen struct and its run method using generics and trait bounds
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// A Button struct that implements the Draw trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

// Another crate using gui and implementing the Draw trait on a SelectBox struct
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

// Using trait objects to store values of different types that implement the same trait
use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
