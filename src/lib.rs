#[doc(hidden)]
pub use std::sync::{Once, ONCE_INIT};

#[macro_export]
macro_rules! ifunky {
    ($(pub fn $fun:ident($($arg:ident: $kind:ty),*) -> $ret:ty { $($selector:expr);* })*) => (
        $(
            #[inline(always)]
            pub fn $fun($($arg: $kind),*) -> $ret {
                static SYNCHRO_START: $crate::Once = $crate::ONCE_INIT;
                static mut INDIRECT: fn( $($kind),* ) -> $ret = dispatch;

                fn dispatch($($arg: $kind),*) -> $ret {
                    fn select() -> fn( $($kind),* ) -> $ret {
                        $($selector);*
                    }

                    SYNCHRO_START.call_once(
                        || unsafe { INDIRECT = select() }
                    );


                    $fun($($arg),*)
                }

                (unsafe { INDIRECT })( $($arg),* )
            }
        )*
    );
}

#[cfg(test)]
mod test {
    extern crate rand;
    use std::iter::repeat;

    ifunky! {
        pub fn foo(x: u32) -> u32 {
            if rand::random::<bool>() {
                foo_big as fn(u32) -> u32
            } else {
                foo_bigger as fn(u32) -> u32
            }
        }

        pub fn bar(x: u32) -> u32 {
            if rand::random::<bool>() {
                bar_small as fn(u32) -> u32
            } else {
                bar_smaller as fn(u32) -> u32
            }
        }
    }

    fn foo_big(x: u32) -> u32 {
        x + 1
    }

    fn foo_bigger(x: u32) -> u32 {
        (x + 1) * 2
    }

    fn bar_small(x: u32) -> u32 {
        x - 1
    }

    fn bar_smaller(x: u32) -> u32 {
        (x - 1) / 2
    }

    #[test]
    fn memoizes_foo() {
        let gold_master_foo = foo(3);
        assert!(
            repeat( 3 )
            .map( foo )
            .take( 999 )
            .all(|x| x == gold_master_foo)
        );
    }

    #[test]
    fn memoizes_bar() {
        let gold_master_bar = bar(3);
        assert!(
            repeat( 3 )
            .map( bar )
            .take( 999 )
            .all(|x| x == gold_master_bar)
        );
    }
}
