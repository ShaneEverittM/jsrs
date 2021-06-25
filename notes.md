## Plans
- At some point will need to make a VM module. This will handle a few things like:
  - Garbage collection (😬)
  - Stack and heap management
  - Call frames (~analogous to interpreter scope stack)
  - Exception handling
  - Stack unwinding
- Might want to implement a REPL, shouldn't be too bad, but will require maybe some special interpreter support? Might be
  able to just use standard `Interpreter::run` and `IRNode::evaluate` on each incoming line.
- Argument completion from command line.
- Bytecode VM!
  - Should come after having a full AST walker implementation
  - Will depend on some functionality of the yet to be VM like garbage collection
  - Should be able to simple add a function to IrNode called `emit_bytecode` and just walk the AST like we do now to evaluate

## Problems

## Question

## Next Time
- Replace *lots* of panics, expects, unwraps, etc. with exceptions. Not all though, some are missing features.
- Make Interpreter generic over its output. This can allow it to integrate into both testing and a repl. Not sure about this one, may be fine as is.