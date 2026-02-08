
#[macro_export]
macro_rules! raise_error {
    ($fmt:expr) => { panic!(concat!("[QIVM Internal Error] ", $fmt)) };
    ($fmt:expr, $($arg:tt)*) => {
        panic!(concat!("[QIVM Internal Error] ", $fmt), $($arg)*)
    };
}

pub trait Unwrap<T> {
    fn unwrap(self) -> T;
}

#[macro_export]
macro_rules! unwrap {
    ($expr:expr => $enum:path) => {{
        if let $enum(item) = $expr {
            item
        } else {
            unreachable!()
        }
    }};
}

#[macro_export]
macro_rules! try_unwrap {
    ($expr:expr => $enum:path) => {{
        if let $enum(item) = $expr {
            Some(item)
        } else {
            None
        }
    }};

    ($expr:expr => $first:path $(=> $followings:path)*) => {{
        if let $first(item) = $expr {
            try_unwrap!(item $(=> $followings)*)
        } else {
            None
        }
    }};
}

#[macro_export]
macro_rules! use_enum {
    ($($enum_ident:path),*) => {
        $(use $enum_ident::*);*;
    };
}

#[macro_export]
macro_rules! dispatch {
    ($variable:expr; $( $variant:ident )|* => |$item:ident| $exec:expr) => {
        $crate::dispatch!(
            @foreach_variant $variable;
            $( $variant($item) => $exec ),*
        )
    };
    ($variable:expr; $( $variant:ident )|* => |ref $item:ident| $exec:expr) => {
        $crate::dispatch!(
            @foreach_variant $variable;
            $( $variant(ref $item) => $exec ),*
        )
    };
    ($variable:expr; $( $variant:ident )|* => |ref mut $item:ident| $exec:expr) => {
        $crate::dispatch!(
            @foreach_variant $variable;
            $( $variant(ref mut $item) => $exec ),*
        )
    };
    (@foreach_variant $variable:expr; $( $pattern:pat => $exec:expr ),*) => {
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::redundant_clone)]
        match $variable {
            $( $pattern => $exec, )*
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}

#[macro_export]
macro_rules! unwrap_variant {
    // single line
    ($from:ident => $to:ident::$variant:ident) => {
        impl Unwrap<$from> for $to {
            fn unwrap(self) -> $from {
                unwrap!(self => $to::$variant)
            }
        }
    };
    // multi-lines
    ($($from:ident => $to:ident::$variant:ident;)+) => {
        $(unwrap_variant!($from => $to::$variant);)+
    }
}

#[macro_export]
macro_rules! into_variant {

    // single line
    ($type:ident => $($enum:ident::$variant:ident)=>+) => {
        into_variant!(@iter_self $type, $($enum::$variant),+);
    };
    // multi-lines
    ($($type:ident => $($enum:ident::$variant:ident)=>+;)+) => {
        $(into_variant!(@iter_self $type, $($enum::$variant),+);)+
    };

    (@iter_self $type:ident, $($enum:ident::$variant:ident),+$(,)?) => {
        into_variant!(@impl $type, $($enum::$variant),*);
        into_variant!(@drop_last $type [$($enum::$variant),*]);
    };

    (@drop_last
        $type:ident
        [$first_e:ident::$first_v:ident, $($tail_e:ident::$tail_v:ident),+]
        $($keep_e:ident::$keep_v:ident),*$(,)?
    ) => {
        into_variant! { @drop_last
            $type [$($tail_e::$tail_v),*] $first_e::$first_v, $($keep_e::$keep_v),*
        }
    };
    (@drop_last
        $type:ident [$first_e:ident::$first_v:ident]
        $($keep_e:ident::$keep_v:ident),+$(,)?
    ) => {
        into_variant!(@drop_rev $type [$($keep_e::$keep_v),*,]);
    };
    (@drop_last $type:ident [$first_e:ident::$first_v:ident] $(,)?) => ();

    (@drop_rev
        $type:ident
        [$first_e:ident::$first_v:ident, $($tail_e:ident::$tail_v:ident),* $(,)?]
        $($keep_e:ident::$keep_v:ident),*$(,)?
    ) => {
        into_variant! { @drop_rev
            $type [$($tail_e::$tail_v),*,] $first_e::$first_v, $($keep_e::$keep_v),*
        }
    };
    (@drop_rev $type:ident [$(,)?] $($keep_e:ident::$keep_v:ident),*$(,)?) => {
        into_variant!(@iter_self $type, $($keep_e::$keep_v),*);
    };

    (@impl $type:ident, $($enums:ident::$variant:ident),+) => {
        impl From<$type> for into_variant!(@skip_to_last $($enums),*) {
            fn from(value: $type) -> Self {
                // pass `value` as a macro arg to avoid macro hygiene
                into_variant!(@construct value | $($enums::$variant),*)
            }
        }
    };

    (@skip_to_last $last:ident) => ( $last );
    (@skip_to_last $type:ident, $($types:ident),+) => {
        into_variant!(@skip_to_last $($types),*)
    };

    (@construct $ident:ident | $path:path) => { $path($ident) };
    (@construct $ident:ident | $first:path, $($rest:path),*) => {
        // init
        into_variant!(@reverse $ident | [$($rest),*] $first,)
    };
    (@reverse $ident:ident | [$first:path, $($rest:path),*] $($reversed:path),*,) => {
        // recursion
        into_variant!(@reverse $ident | [$($rest),*] $first, $($reversed),*,)
    };
    (@reverse $ident:ident | [$first:path] $($reversed:path),*,) => {
        // base case
        into_variant!(@rev_cons $ident | $first, $($reversed),* )
    };

    (@rev_cons $ident:ident | $path:path) => ( $path($ident) );
    (@rev_cons $ident:ident | $path:path, $($paths:path),+) => {
        $path(into_variant!(@rev_cons $ident | $($paths),*))
    };
}
