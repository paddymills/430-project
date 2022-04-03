
use requestty::{Answer, Answers};

// Convenience trait to unwrap answers
// answers are actually a HashMap
// so they return Option<Answer<value>>
// 
// this is highly unsafe and only works
// because I know the key I am getting is
// in the answer
pub trait AnswerValue {
    type Int;
    type Float;

    fn get_int(self: &Self, key: &str) -> Self::Int;
    fn get_float(self: &Self, key: &str) -> Self::Float;
    fn get_str(self: &Self, key: &str) -> String;
}

impl AnswerValue for Answers {

    type Int = u32;
    type Float = f32;

    fn get_int(self: &Self, key: &str) -> Self::Int {
        match self.get(key) {
            Some(val) => match val {
                Answer::Int(v) => *v as Self::Int,
                _ => unreachable!(),
            },
            None => unreachable!("")
        }
    }

    fn get_float(self: &Self, key: &str) -> Self::Float {
        match self.get(key) {
            Some(val) => match val {
                Answer::Float(v) => *v as Self::Float,
                _ => unreachable!(),
            },
            None => unreachable!("")
        }
    }

    fn get_str(self: &Self, key: &str) -> String {
        match self.get(key) {
            Some(val) => match val {
                Answer::String(v) => String::from(v),
                _ => unreachable!(),
            },
            None => unreachable!("")
        }
    }

}
