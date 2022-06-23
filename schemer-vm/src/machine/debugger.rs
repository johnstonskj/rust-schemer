/*!
One-line description.

More detailed description, with

# Example

*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "future")]
pub type BreakpointHandler = Box<dyn Fn(&dyn DebuggableMachine, usize)>;

#[cfg(feature = "future")]
pub trait Breakpoints: Machine {
    fn set_global_breakpoint_handler(&mut self, handler: BreakpointHandler);
    fn current_instruction(&self) -> usize;
    fn set_breakpoint(&mut self, instruction: usize) -> Result<usize, Error>;
    fn set_breakpoint_now(&mut self) -> Result<usize, Error> {
        self.set_breakpoint(self.current_instruction())
    }
    fn set_breakpoint_with_handler(
        &mut self,
        instruction: usize,
        handler: BreakpointHandler,
    ) -> Result<usize, Error>;
    fn set_breakpoint_now_with_handler(
        &mut self,
        handler: BreakpointHandler,
    ) -> Result<usize, Error> {
        self.set_breakpoint_with_handler(self.current_instruction(), handler)
    }
    fn remove_breakpoint(&mut self, breakpoint: usize);
}

#[cfg(feature = "future")]
pub type TraceHandler = Box<dyn Fn(&dyn TraceableMachine, &Identifier, &Datum)>;

#[cfg(feature = "future")]
pub trait Tracing: Machine {
    fn set_global_trace_handler(&mut self, handler: TraceHandler);
    fn set_trace(&mut self, name: &Identifier) -> Result<usize, Error>;
    fn set_trace_with_handler(
        &mut self,
        name: &Identifier,
        handler: TraceHandler,
    ) -> Result<usize, Error>;
    fn remove_trace(&mut self, trace: usize);
    fn is_bound(&self, name: &Identifier) -> bool;
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
