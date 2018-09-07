use std::marker;
use std::{ffi::CString, ptr};
use v8_sys::{v8, v8::platform};
// use v8;

fn main() {
    unsafe {
        // let startup_data_dir = CString::new("D:/documents/dev/enigma-deps/v8-rs/target/debug/").unwrap();
        let startup_data_dir = CString::new("./").unwrap();
        v8::V8_InitializeExternalStartupData(startup_data_dir.as_ptr());
        let p: *mut v8::TracingController = ptr::null_mut();
        let platform = platform::CreateDefaultPlatform(
            0,
            platform::IdleTaskSupport_kDisabled,
            platform::InProcessStackDumping_kDisabled,
            p,
        );
        v8::V8_InitializePlatform(platform);
        v8::V8_Initialize();
        println!("Initialized");

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
        let mut isolate =
            ptr::NonNull::new(v8::Isolate::New(&params)).expect("Could not create Isolate");
        isolate.as_mut().Enter();

        println!("isolate created");
        let hs = v8_sys::v8::HandleScope::new(isolate.as_ptr());

        // Create a new context of execution
        // context creation currently crashes the application for some reason.
        let cb = v8::DeserializeInternalFieldsCallback {
            callback: None,
            data: ptr::null_mut(),
        };
        let h = ::v8::handle::MaybeLocal::empty().into_raw();
        let i = ::v8::handle::MaybeLocal::empty().into_raw();
        println!("Just before calling Context constructor in V8, crashes after this.");
        let c = v8::Context::New(isolate.as_ptr(), ptr::null_mut(), h, i, cb);
        // let context = v8::Context::new(&isolate);
        println!("Created context");
        /*
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
         */
    }
}
