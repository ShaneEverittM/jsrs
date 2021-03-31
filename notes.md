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

## Problem

## Question

## Next Time
- Exception system. Make an exception error type and have evaluate return a Result<Option<Value>, Exception> and use ?
  to bubble up as needed. Can have a function that called Interpreter::run() to start and have it catch all unhandled
  exceptions.
- Organize imports.