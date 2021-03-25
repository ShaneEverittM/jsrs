## Plans
- At some point will need to make a VM module. This will handle a few things like:
    - Garbage collection (😬)
    - Stack and heap management
    - Call frames (~analogous to interpreter scope stack)
    - Exception handling
    - Stack unwinding
- Will want to implement a REPL, shouldn' be to bad, but will require maybe some special interpreter support? Might be able to just use standard `Interpreter::run` and `IRNode::evaluate` on each incoming line.
- Argument completion from command line.

## Problems
- How to handle objects pointing to same memory? This is generally disallowed in Rust.
    - Go fully unsafe and write a wrapper class around object that holds a raw pointer underneath?
    - ~~Steal~~ Be inspired by `evmap::ShallowCopy?`
    
## Questions
- Am I using clone too much?

## Next Time
- Improve variable lookup and how variable are stored. Maybe rewrite scopes? Maybe a Scope node and a current frame should be different concepts. So a Scope is the IR repr but a Frame is a runtime concept. A Scope is just a container for statements and a Frame could hold context? Or maybe just have easier lookup within a scope than just a list of decls.
- After that, try for loops based off of branch "for_loop".