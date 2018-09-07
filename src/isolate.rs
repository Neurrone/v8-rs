//! Heap and execution isolation.
//!
//! # Usage
//!
//! Construct a new isolate with default settings by doing `Isolate::new()`.
//!

use std::{ffi::CString, ptr, sync};
use v8_sys::{v8, v8::platform};

static INITIALIZE: sync::Once = sync::ONCE_INIT;

/// Isolate represents an isolated instance of the V8 engine.
///
/// V8 isolates have completely separate states.  Objects from one isolate must not be used in other
/// isolates.  The embedder can create multiple isolates and use them in parallel in multiple
/// threads.  An isolate can be entered by at most one thread at any given time.  The
/// Locker/Unlocker API must be used to synchronize.
pub struct Isolate(ptr::NonNull<v8::Isolate>);

#[must_use]
pub struct Scope<'i>(&'i mut Isolate);

impl Isolate {
    /// Creates a new isolate.
    pub fn new() -> Isolate {
        ensure_initialized();
        let raw = unsafe {
            // let mut params: v8::Isolate_CreateParams = mem::zeroed();
            let params = v8::Isolate_CreateParams {
                entry_hook: None,
                code_event_handler: None,
                constraints: v8::ResourceConstraints {
                    max_semi_space_size_in_kb_: 0,
                    max_old_space_size_: 0,
                    max_executable_size_: 0,
                    stack_limit_: ptr::null_mut(),
                    code_range_size_: 0,
                    max_zone_pool_size_: 0,
                },
                snapshot_blob: ptr::null_mut(),
                counter_lookup_callback: None,
                create_histogram_callback: None,
                add_histogram_sample_callback: None,
                external_references: ptr::null_mut(),
                only_terminate_in_safe_scope: false,
                allow_atomics_wait: true,
                array_buffer_allocator: v8::ArrayBuffer_Allocator::NewDefaultAllocator(),
            };
            ptr::NonNull::new(v8::Isolate::New(&params)).expect("Could not create Isolate")
        };
        /*
        unsafe {
            raw.as_mut().SetCaptureStackTraceForUncaughtExceptions(true, 1024, v8::StackTrace_StackTraceOptions_kDetailed);
        }
        */
        Isolate(raw)
    }

    pub fn scope(&mut self) -> Scope {
        unsafe { self.0.as_mut().Enter() };
        Scope(self)
    }

    pub fn enter(&mut self) {
        unsafe { self.0.as_mut().Enter() };
    }

    /// Returns the underlying raw pointer behind this isolate.
    pub fn as_ptr(&self) -> *mut v8::Isolate {
        self.0.as_ptr()
    }

    /*
    /// Returns the context bound to the current thread for this isolate.
    ///
    /// A context will be bound by for example `Context::make_current`, or while inside of a
    /// function callback.
    pub fn current_context(&self) -> Option<context::Context> {
        unsafe {
            let raw = self.isolate(self.as_raw()).as_mut();
            raw.map(|r| context::Context::from_raw(self, r))
        }
    }
    */
}

/*
impl fmt::Debug for Isolate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Isolate({:?}, {:?})",
            unsafe { self.0.as_ref() },
            self.data()
        )
    }
}
*/

impl Drop for Isolate {
    fn drop(&mut self) {
        unsafe {
            self.0.as_mut().Dispose();
        }
    }
}

impl<'i> Scope<'i> {
    pub fn isolate(&self) -> &Isolate {
        &self.0
    }

    pub fn isolate_mut(&mut self) -> &mut Isolate {
        &mut self.0
    }
}

impl<'i> Drop for Scope<'i> {
    fn drop(&mut self) {
        unsafe { (self.0).0.as_mut().Exit() }
    }
}

fn ensure_initialized() {
    INITIALIZE.call_once(|| {
        unsafe {
            // v8::V8_InitializeICUDefaultLocation(ptr::null());
            // let startup_data_dir = CString::new("D:/documents/dev/enigma-deps/v8-rs/target/debug/").unwrap();
            let startup_data_dir = CString::new("./").unwrap();
            v8::V8_InitializeExternalStartupData(startup_data_dir.as_ptr());
            let platform = platform::CreateDefaultPlatform(
                0,
                platform::IdleTaskSupport_kDisabled,
                platform::InProcessStackDumping_kDisabled,
                ptr::null_mut(),
            );
            v8::V8_InitializePlatform(platform);
            // TODO: implement some form of cleanup
            v8::V8_Initialize();
            println!("Initialized");
        }
    });
}
