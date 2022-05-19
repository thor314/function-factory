use std::fmt::Display;

fn dyn_greeter_factory<'a, T: Display + ?Sized>(
    greeting: &'a T,
    name: &'a T,
    comment: &'a T,
) -> Box<dyn Fn() -> String + 'a> {
    Box::new(move || format!("{}, {} {}", greeting, name, comment))
}

fn impl_greeter_factory<'a, T: Display + ?Sized>(
    greeting: &'a T,
    name: &'a T,
    comment: &'a T,
    // this return type can be impl or a trait object
) -> impl Fn() -> String + 'a {
    move || format!("{}, {} {}", greeting, name, comment)
}

fn dyn_caller<'a, T: Display + ?Sized>(
    // this argument type can take an opaque type argument
    greeter: impl Fn(&'a T, &'a T, &'a str) -> Box<dyn Fn() -> String>,
    greeting: &'a T,
    name: &'a T,
) -> Box<dyn Fn() -> String> {
    greeter(
        greeting,
        name,
        &"I'm slower but more flexible at runtime :)",
    )
}

fn impl_caller<'a, T: Display + ?Sized, U: Fn() -> String>(
    // not allowed: can't return a function that returns a function with an opaque type
    // greeter: impl Fn(&'a T, &'a T, &'a str) -> impl Fn() -> String,
    // but this is okay, if we update the factory to return its closure in a box:
    // greeter: impl Fn(&'a T, &'a T, &'a str) -> Box<dyn Fn() -> String>,
    // We can actually use generics to solve our problem (instead of create more here:)
    greeter: impl Fn(&'a T, &'a T, &'a str) -> U,
    greeting: &'a T,
    name: &'a T,
) -> impl Fn() -> String {
    greeter(
        greeting,
        name,
        &"I'm faster but take more space because of monomorphization :)",
    )
}

fn main() {
    let dyn_ = dyn_caller(Box::new(dyn_greeter_factory), "hi", "becc");
    let dyn_call = dyn_();
    println!("{}", dyn_call);

    let impl_ = impl_caller(impl_greeter_factory, "hi", "friend");
    let impl_call = impl_();
    println!("{}", impl_call);
}
