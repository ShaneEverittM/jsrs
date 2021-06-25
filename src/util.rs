use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

use crate::runtime::Object;

pub fn make_indent(indent: u32) -> String {
    let mut indents = String::new();
    for _ in 0..indent {
        indents.push(' ');
    }
    indents
}

pub fn get_input() -> String {
    let file_name = std::env::args().nth(1);

    match file_name {
        None => {
            // read from stdin
            let mut buffer = String::new();
            let stdin = std::io::stdin();
            let mut lock = stdin.lock();
            lock.read_to_string(&mut buffer).unwrap();
            buffer
        }
        Some(file_name) => {
            // read from file
            std::fs::read_to_string(&file_name).unwrap()
        }
    }
}

pub fn wrap_object(obj: Box<dyn Object>) -> Rc<RefCell<Box<dyn Object>>> {
    Rc::new(RefCell::new(obj))
}

pub trait OnSuccess<T, E> {
    fn finally<F: FnOnce(T)>(self, op: F) -> Result<(), E>;
}

impl<T, E> OnSuccess<T, E> for Result<T, E> {
    /// Calls `op` if the result is [`Ok`], otherwise returns the [`Err`] value of `self`.
    ///
    ///
    /// This function can be used for control flow based on `Result` values. Like [`and_then`], but
    /// does not require a value to be returned. Use to finish a chain of fallible operations with
    /// an infallible operation.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use jsrs::util::OnSuccess;
    /// fn sq(x: u32) -> Result<u32, u32> { Ok(x * x) }
    /// fn err(x: u32) -> Result<u32, u32> { Err(x) }
    /// fn last(x: u32) -> u32 { x + 1 }
    ///
    ///
    /// assert_eq!(Ok(2).and_then(sq).finally(sq), Ok(()));
    /// ```
    fn finally<F: FnOnce(T)>(self, op: F) -> Result<(), E> {
        match self {
            Ok(t) => {
                op(t);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
