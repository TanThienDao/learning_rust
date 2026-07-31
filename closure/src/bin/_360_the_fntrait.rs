fn execute_thrice<F>(procedure: F)
where
    F: Fn(),
{
    procedure();
    procedure();
    procedure();
}
fn main() {
    let mut bosses = vec!["Boss Eyes", "Boss West"];
    let closure = || println!("I am a closure");
    execute_thrice(closure);
    /* let closure2 = || {
        //let employee = bosses;
        bosses.push("Boss East");

    };*/
    // execute_thrice(closure2); // fn is a trickest one you can not pass a closure that captures a variable by move to a function that takes Fn() because Fn() requires the closure to be callable multiple times without consuming its environment. In this case, the closure captures bosses by move, which means it can only be called once.
}
