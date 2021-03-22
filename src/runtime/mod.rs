pub use function::Function;
pub use interpreter::Interpreter;
pub use object::Object;
pub use object::Type as ObjectType;
pub use value::Value;

mod function;
mod interpreter;
mod object;
mod value;
mod literal_object;
mod string;
