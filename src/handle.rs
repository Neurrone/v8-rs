use super::isolate;
use std::convert;
use std::marker;
use std::mem;
use std::ops;
use std::ptr;
use v8_sys::v8;

#[derive(Debug)]
pub struct Scope<'i>(
    pub v8::HandleScope,
    pub marker::PhantomData<&'i isolate::Isolate>,
);

#[derive(Debug, Copy, Clone)]
pub struct Local<'i, 's, A>(v8::Local<A>, marker::PhantomData<&'s Scope<'i>>)
where
    'i: 's;

#[derive(Debug, Copy, Clone)]
pub struct MaybeLocal<'i, 's, A>(v8::MaybeLocal<A>, marker::PhantomData<&'s Scope<'i>>)
where
    'i: 's;

/*
#[derive(Debug, Copy, Clone)]
pub struct Eternal<'i, A>(v8::Eternal<A>, marker::PhantomData<&'i isolate::Isolate>);
*/

#[derive(Debug)]
pub struct Persistent<'i, A>(v8::Persistent<A>, marker::PhantomData<&'i isolate::Isolate>);

impl<'i, 's, A> Local<'i, 's, A> {
    pub unsafe fn new<B>(value: v8::Local<B>) -> Local<'i, 's, A> {
        assert_eq!(mem::size_of::<B>(), mem::size_of::<A>());
        Local(mem::transmute(value), marker::PhantomData)
    }

    pub fn into_raw(self) -> v8::Local<A> {
        self.0
    }
}

impl<'i, 's, A> convert::From<v8::Local<A>> for Local<'i, 's, A> {
    fn from(other: v8::Local<A>) -> Self {
        Local(other, marker::PhantomData)
    }
}

impl<'i, 's, A> ops::Deref for Local<'i, 's, A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.val_.as_ref() }.unwrap()
    }
}

impl<'i, 's, A> ops::DerefMut for Local<'i, 's, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.val_.as_mut() }.unwrap()
    }
}

impl<'i, 's, A> MaybeLocal<'i, 's, A> {
    pub fn empty() -> MaybeLocal<'i, 's, A> {
        MaybeLocal(
            v8::MaybeLocal {
                val_: ptr::null_mut(),
                _phantom_0: marker::PhantomData,
            },
            marker::PhantomData,
        )
    }

    pub fn into_raw(self) -> v8::MaybeLocal<A> {
        self.0
    }
}
