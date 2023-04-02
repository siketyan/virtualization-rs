pub(crate) trait ObjcId {
    fn objc_id(&self) -> Id;
}

macro_rules! alloc {
    ($name: ident) => {{
        use objc::{class, msg_send, sel, sel_impl};
        #[allow(unused_unsafe)]
        let i: crate::foundation::Id = (unsafe { msg_send![class!($name), alloc] });
        i
    }};
}

macro_rules! new {
    ($name: ident) => {{
        use objc::{class, msg_send, sel, sel_impl};
        #[allow(unused_unsafe)]
        unsafe {
            objc::rc::StrongPtr::new(msg_send![class!($name), new])
        }
    }};
}

macro_rules! retain {
    ($obj: expr, $name: ident) => {{
        use objc::{msg_send, sel, sel_impl};
        unsafe { objc::rc::StrongPtr::retain(msg_send![$obj, $name]) }
    }};
}

macro_rules! instancetype {
    ($obj: expr, $($name: ident : $arg: expr)+) => {{
        use objc::{msg_send, sel, sel_impl};
        unsafe { objc::rc::StrongPtr::new(msg_send![$obj, $($name: $arg)+]) }
    }}
}

macro_rules! property {
    ($name: ident : $ty: ty {
        $(get : $gvis: vis $gname: ident $(< $($gparam: ty : $($greq: ty)+)+ >)? ;)?
        $(set : $svis: vis $sname: ident $(< $($sparam: ty : $($sreq: ty)+)+ >)? ;)?
    }) => {
        $($crate::concat_idents!(fn_name = get_, $name {
            $gvis fn fn_name $(< $($gparam : $($greq)+)+ >)? (&self) -> $ty {
                $ty($crate::objc::retain![self.id(), $gname])
            }
        });)?

        $($crate::concat_idents!(fn_name = set_, $name {
            $svis fn fn_name $(< $($sparam : $($sreq)+)+ >)? (&mut self, $name: $ty) {
                use objc::{msg_send, sel, sel_impl};
                unsafe { msg_send![self.id(), $sname: $name.id()] }
            }
        });)?
    };
}

macro_rules! constructor {
    ($(#[$attrs: meta])* $ty: ident) => {
        impl $ty {
            $(#[$attrs])*
            pub fn new() -> Self {
                Self($crate::objc::new!($ty))
            }
        }
    };
}

macro_rules! protocol {
    ($proto: ty, $impl: ty $(,)?) => {
        impl $proto for $impl {
            fn id(&self) -> Id {
                *self.0
            }
        }
    };
}

use crate::foundation::Id;
pub(crate) use {alloc, constructor, instancetype, new, property, protocol, retain};
