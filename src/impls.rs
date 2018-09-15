use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::core::*;

pub fn identity<T>() -> Transmute<T, T, True> {
    unsafe { Transmute::new_unchecked() }
}

#[macro_export(local_inner_macros)]
macro_rules! declare {
    ($pub:vis fn $name:ident; $from:ty, $to:ty) => {
        $pub fn $name() -> Transmute<$from, $to> {
            unsafe { Transmute::new_unchecked() }
        }
    };
    ($pub:vis fn $name:ident<$($lifetimes:lifetime)*>; $from:ty, $to:ty) => {
        $pub fn $name<$($lifetimes)*>() -> Transmute<$from, $to> {
            unsafe { Transmute::new_unchecked() }
        }
    };
    ($pub:vis fn $name_forward:ident, $name_backward:ident; $from:ty, $to:ty) => {
        $pub fn $name_forward() -> Transmute<$from, $to, True> {
            unsafe { Transmute::new_unchecked() }
        }

        $pub fn $name_backward() -> Transmute<$to, $from, True> {
            unsafe { Transmute::new_unchecked() }
        }
    };
}

declare!(pub fn i8_u8, u8_i8; i8, u8);
declare!(pub fn i16_u16, u16_i16; i16, u16);
declare!(pub fn i32_u32, u32_i32; i32, u32);
declare!(pub fn i64_u64, u64_i64; i64, u64);
declare!(pub fn isize_usize, usize_isize; isize, usize);

declare!(pub fn str_bytes<'a>; &'a str, &'a [u8]);
declare!(pub fn string_bytes; String, Vec<u8>);

pub fn slice<'a, T, U, X>(_items: Transmute<T, U, X>) -> Transmute<&'a T, &'a U, X> {
    unsafe { Transmute::new_unchecked() }
}

pub fn vec<T, U, X>(_items: Transmute<T, U, X>) -> Transmute<Vec<T>, Vec<U>, X> {
    unsafe { Transmute::new_unchecked() }
}

pub fn btree_map_values<K, T, U, X>(_values: Transmute<T, U, X>)
    -> Transmute<BTreeMap<K, T>, BTreeMap<K, U>, X>
{
    unsafe { Transmute::new_unchecked() }
}

pub fn hash_map_values<K, T, U, S, X>(_values: Transmute<T, U, X>)
    -> Transmute<HashMap<K, T, S>, HashMap<K, U, S>, X>
{
    unsafe { Transmute::new_unchecked() }
}
