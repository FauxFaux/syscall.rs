// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export]
macro_rules! syscall {
    ($nr:ident)
        => ( ::syscall::syscall0(
                ::syscall::nr::$nr) );

    ($nr:ident, $a1:expr)
        => ( ::syscall::syscall1(
                ::syscall::nr::$nr,
                $a1 as uint) );

    ($nr:ident, $a1:expr, $a2:expr)
        => ( ::syscall::syscall2(
                ::syscall::nr::$nr,
                $a1 as uint, $a2 as uint) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr)
        => ( ::syscall::syscall3(
                ::syscall::nr::$nr,
                $a1 as uint, $a2 as uint, $a3 as uint) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr)
        => ( ::syscall::syscall4(
                ::syscall::nr::$nr,
                $a1 as uint, $a2 as uint, $a3 as uint,
                $a4 as uint) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr)
        => ( ::syscall::syscall5(
                ::syscall::nr::$nr,
                $a1 as uint, $a2 as uint, $a3 as uint,
                $a4 as uint, $a5 as uint) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr)
        => ( ::syscall::syscall6(
                ::syscall::nr::$nr,
                $a1 as uint, $a2 as uint, $a3 as uint,
                $a4 as uint, $a5 as uint, $a6 as uint) );
}
