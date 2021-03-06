// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: -Z identify_regions
// ignore-tidy-linelength

// Binding the borrow's subject outside the loop does not increase the
// scope of the borrow.

fn main() {
    let mut a;
    loop {
        a = true;
        let b = &a;
        if a { break; }
        let c = &a;
    }
}

// END RUST SOURCE
// START rustc.node4.SimplifyCfg-qualify-consts.after.mir
//     let mut _0: ();
//     let mut _1: bool;
//     let _3: &'26_1rce bool;
//     let _7: &'26_3rce bool;
//     let mut _2: ();
//     let mut _4: ();
//     let mut _5: bool;
//
//     bb0: {
//         StorageLive(_1);
//         goto -> bb1;
//     }
//     bb1: {
//         _1 = const true;
//         StorageLive(_3);
//         _3 = &'26_1rce _1;
//         StorageLive(_5);
//         _5 = _1;
//         switchInt(_5) -> [0u8: bb3, otherwise: bb2];
//     }
//     bb2: {
//         _0 = ();
//         StorageDead(_5);
//         StorageDead(_3);
//         EndRegion('26_1rce);
//         StorageDead(_1);
//         return;
//     }
//     bb3: {
//         _4 = ();
//         StorageDead(_5);
//         StorageLive(_7);
//         _7 = &'26_3rce _1;
//         _2 = ();
//         StorageDead(_7);
//         EndRegion('26_3rce);
//         StorageDead(_3);
//         EndRegion('26_1rce);
//         goto -> bb1;
//     }
// END rustc.node4.SimplifyCfg-qualify-consts.after.mir
