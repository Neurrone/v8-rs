//! Execution contexts and sandboxing.
use super::{handle, isolate};
use std::ptr;
use v8_sys::v8;
// use value;

/// A sandboxed execution context with its own set of built-in objects and functions.
#[derive(Debug)]
pub struct Context(v8::Context);

/// A guard that keeps a context bound while it is in scope.
#[must_use]
pub struct Scope<'c>(&'c mut Context);

impl Context {
    /// Creates a new context and returns a handle to the newly allocated context.
    pub fn new<'i, 's>(
        // _scope: &'s handle::Scope,
        isolate: &'i isolate::Isolate
    ) -> handle::Local<'i, 's, Context> {
        unsafe {
            println!("in context::new()");
            let cb = v8::DeserializeInternalFieldsCallback {
                callback: None,
                data: ptr::null_mut(),
            };
            println!("callback created");
            let h = handle::MaybeLocal::empty().into_raw();
            let i = handle::MaybeLocal::empty().into_raw();
            let c = v8::Context::New(isolate.as_ptr(), ptr::null_mut(), h, i, cb);
            println!("About to return local handle");
            handle::Local::new(c)
        }
    }

    /// Binds the context to the current scope.
    ///
    /// Within this scope, functionality that relies on implicit contexts will work.
    pub fn scope(&mut self) -> Scope {
        unsafe {
            self.0.Enter();
        }
        Scope(self)
    }

    /*
    /// Returns the global proxy object.
    ///
    /// Global proxy object is a thin wrapper whose prototype points to actual context's global
    /// object with the properties like Object, etc. This is done that way for security reasons (for
    /// more details see https://wiki.mozilla.org/Gecko:SplitWindow).
    ///
    /// Please note that changes to global proxy object prototype most probably would break VM---v8
    /// expects only global object as a prototype of global proxy object.
    ///
    pub fn global(&self) -> handle::Local<value::Object> {
        unsafe {
            handle::Local::new(self.0.Global())
        }
    }
    */
}

impl<'c> Scope<'c> {
    pub fn context(&self) -> &Context {
        &self.0
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}

impl<'c> Drop for Scope<'c> {
    fn drop(&mut self) {
        unsafe { (self.0).0.Exit() }
    }
}
