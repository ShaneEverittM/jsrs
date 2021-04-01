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

## Question

## Next Time
- Make Interpreter generic over its output. This can allow it to integrate into both testing and a repl.