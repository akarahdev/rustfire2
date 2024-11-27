use crate::api::items::number::Number;
use crate::api::items::string::String;

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number::new(&self.to_string())
    }
}

impl Into<Number> for i64 {
    fn into(self) -> Number {
        Number::new(&self.to_string())
    }
}

impl Into<Number> for f32 {
    fn into(self) -> Number {
        Number::new(&self.to_string())
    }
}

impl Into<Number> for f64 {
    fn into(self) -> Number {
        Number::new(&self.to_string())
    }
}

impl Into<String> for std::string::String {
    fn into(self) -> String {
        String::new(&self.to_string())
    }
}