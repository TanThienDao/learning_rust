/// Trait Hierarchy
/// FnOnce
///     Closure captures value by move
///     (transferring ownership)
///     Closure will be invoked once
///
/// FnMut
///     Captures values by mutable reference
///     Closure can be invoked multiple times
///
/// Fn
///     Closure captures values by immutable reference (read-only)
///     or does not capture anything at all
///     Closure can be invoked multiples times.

fn main() {}
