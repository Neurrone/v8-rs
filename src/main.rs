use std::marker;
use v8;
fn main() {
    println!("creating");
    let mut isolate = v8::Isolate::new();
    isolate.scope();
    unsafe {
        println!("isolate created");
        let hs = v8_sys::v8::HandleScope::new(isolate.as_ptr());
        let scope = v8::handle::Scope(hs, marker::PhantomData);
        println!("{:#?}", isolate.as_ptr());    
        // Create a new context of execution
        // context creation currently crashes the application for some reason.
        let context = v8::Context::new(&scope, &isolate);
        println!("Created context");
        // Load the source code that we want to evaluate
        // let source = value::String::from_str(&isolate, "'Hello, ' + 'World!'");
        // Compile the source code.  `unwrap()` panics if the code is invalid,
        // e.g. if there is a syntax  error.
        // let script = v8::Script::compile(&isolate, &context, &source).unwrap();
        // Run the compiled script.  `unwrap()` panics if the code threw an
        // exception.
        // let result = script.run(&context).unwrap();
        // Convert the result to a value::String.
        // let result_str = result.to_string(&context);
    }
}