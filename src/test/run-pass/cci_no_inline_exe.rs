// xfail-fast - check-fast doesn't understand aux-build
// aux-build:cci_no_inline_lib.rs

extern mod cci_no_inline_lib;
use cci_no_inline_lib::iter;

fn main() {
    // Check that a cross-crate call function not marked as inline
    // does not, in fact, get inlined.  Also, perhaps more
    // importantly, checks that our scheme of using
    // sys::frame_address() to determine if we are inlining is
    // actually working.
    //let bt0 = sys::frame_address();
    //debug!("%?", bt0);
    do iter(~[1u, 2u, 3u]) |i| {
        io::print(fmt!("%u\n", i));

        //let bt1 = sys::frame_address();
        //debug!("%?", bt1);

        //assert bt0 != bt1;
    }
}
