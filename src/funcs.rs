use serde::ser::{Serialize};
use serde_json::to_string;

trait JsonPanic {
    fn json(&self) -> String;
}

default impl<T: ?Sized + Serialize> JsonPanic for T {
    fn json(&self) -> String {
        to_string(self).unwrap()
    }
}