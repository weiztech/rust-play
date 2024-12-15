use std::fmt;
use std::fmt::{Debug, Formatter};

pub trait FromRef<T> {
    /// Converts to this type from a reference to the input type.
    fn from_ref(input: &T) -> Self;
}

impl<T> FromRef<T> for T
where
    T: Clone,
{
    fn from_ref(input: &T) -> Self {
        input.clone()
    }
}

pub trait FromOther<T>: Sized{
    fn from_other(&self, value: &T) -> Self;
}

#[derive(Clone)]
struct Text(String);

#[derive(Clone, Debug)]
struct Numba(usize);

#[derive(Clone, Debug)]
struct Classic(usize);


impl <T> FromOther<T> for Text
where
    T: Send + Sync + Debug + FromRef<T>
{
    fn from_other(&self, value: &T) -> Self {
        println!("Total Str Value {:?}", value);
        let value_clone = T::from_ref(value);
        println!("Clone {:?}", value_clone);
        Self(self.0.to_string())
    }
}

/*
impl <T> FromOther<T> for Text
where
    Classic: FromRef<T>,
    T: Send + Sync + Debug,
{
    fn from_other(&self, value: &T) -> Self {
        println!("Total Str Value {:?}", value);
        let value_clone = Classic::from_ref(value);
        println!("Clone {:?}", value_clone);
        Self(value_clone.0.to_string())
    }
}
*/

impl fmt::Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


fn main() {
    let text = Text(String::from("This is text value"));
    let text_clone = Text::from_ref(&text);
    let num = Numba(10);
    let classic = Classic(10);
    text_clone.from_other(&num);
    text_clone.from_other(&classic);
    // println!("{text} {text_clone}");
}
