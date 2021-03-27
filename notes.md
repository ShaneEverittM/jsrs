## Plans
- At some point will need to make a VM module. This will handle a few things like:
  - Garbage collection (😬)
  - Stack and heap management
  - Call frames (~analogous to interpreter scope stack)
  - Exception handling
  - Stack unwinding
- Might want to implement a REPL, shouldn't be to bad, but will require maybe some special interpreter support? Might be
  able to just use standard `Interpreter::run` and `IRNode::evaluate` on each incoming line.
- Argument completion from command line.

## Problems
- How to handle objects pointing to same memory? This is generally disallowed in Rust.
    - Go fully unsafe and write a wrapper class around object that holds a raw pointer underneath?
    - ~~Steal~~ Be inspired by `evmap::ShallowCopy?`
    - and Rc<RefCell<Object>>> might do the trick!
    
## Questions
- Am I using clone too much?

## Next Time

- If I'm feeling ambitious, take a stab at putting an alias to the global object in the top level scope using Rc and
  RefCell.
- Organize imports.