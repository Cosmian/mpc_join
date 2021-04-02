#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use cosmian_std::scale::{self, println};
use cosmian_std::{prelude::*, test, Column, InputRow, OutputRow};
const BANK_MORTGAGE: u32 = 0;
const BANK_SUBSIDIARY1: u32 = 1;
const BANK_SUBSIDIARY2: u32 = 2;
use ::scale_core::print;
use ::scale_core::*;
type SecretInteger<const K: u64> = scale_std::integer::SecretInteger<K, 40>;
type SecretFixed<const K: u64, const F: u64> = scale_std::fixed_point::SecretFixed<K, F, 40>;
type SecretFloat<const V: u64, const P: u64> = scale_std::floating_point::SecretFloat<V, P, 40>;
#[inline(never)]
fn lomain() {
    ::scale_core::Print::print("##### Reading from players");
    ::scale_core::Print::print("\n");
    loop {
        let mut row_bank_subsidiary_1 = InputRow::read(Player::<BANK_SUBSIDIARY1>);
        let mut id_bank_subsidiary_1 = match row_bank_subsidiary_1.next_col() {
            Some(Column::SecretModp(id)) => id,
            None => {
                ::scale_core::Print::print("done row_bank_subsidiary_1 first");
                ::scale_core::Print::print("\n");
                return;
            }
            _ => {
                {
                    ::scale_core::Print::print("bad format !");
                    unsafe { ::scale_core::__crash() }
                };
                return;
            }
        };
        let mut row_bank_subsidiary_2 = InputRow::read(Player::<BANK_SUBSIDIARY2>);
        let mut id_bank_subsidiary_2 = match row_bank_subsidiary_2.next_col() {
            Some(Column::SecretModp(id)) => id,
            None => {
                ::scale_core::Print::print("done row_bank_subsidiary_2 first");
                ::scale_core::Print::print("\n");
                return;
            }
            _ => {
                {
                    ::scale_core::Print::print("bad format !");
                    unsafe { ::scale_core::__crash() }
                };
                return;
            }
        };
        loop {
            if SecretI64::from(id_bank_subsidiary_1)
                .eq(SecretI64::from(id_bank_subsidiary_2))
                .reveal()
            {
                let mut row_bank_mortgage = OutputRow::new(Player::<BANK_MORTGAGE>);
                row_bank_mortgage.append(id_bank_subsidiary_2);
                row_bank_mortgage.append(
                    row_bank_subsidiary_1
                        .next_col()
                        .unwrap()
                        .into_secret_modp()
                        .expect("value should be i64"),
                );
                row_bank_mortgage.append(
                    row_bank_subsidiary_1
                        .next_col()
                        .unwrap()
                        .into_secret_modp()
                        .expect("value should be i64"),
                );
                row_bank_mortgage.append(
                    row_bank_subsidiary_2
                        .next_col()
                        .unwrap()
                        .into_secret_modp()
                        .expect("value should be i64"),
                );
                row_bank_mortgage.append(
                    row_bank_subsidiary_2
                        .next_col()
                        .unwrap()
                        .into_secret_modp()
                        .expect("value should be i64"),
                );
                break;
            } else if SecretI64::from(id_bank_subsidiary_1)
                .lt(SecretI64::from(id_bank_subsidiary_2))
                .reveal()
            {
                row_bank_subsidiary_1 = InputRow::read(Player::<BANK_SUBSIDIARY1>);
                id_bank_subsidiary_1 = match row_bank_subsidiary_1.next_col() {
                    Some(Column::SecretModp(id)) => id,
                    None => {
                        ::scale_core::Print::print("done row_bank_subsidiary_1 first");
                        ::scale_core::Print::print("\n");
                        return;
                    }
                    _ => {
                        {
                            ::scale_core::Print::print("bad format !");
                            unsafe { ::scale_core::__crash() }
                        };
                        return;
                    }
                };
            } else if SecretI64::from(id_bank_subsidiary_2)
                .lt(SecretI64::from(id_bank_subsidiary_1))
                .reveal()
            {
                row_bank_subsidiary_2 = InputRow::read(Player::<BANK_SUBSIDIARY2>);
                id_bank_subsidiary_2 = match row_bank_subsidiary_2.next_col() {
                    Some(Column::SecretModp(id)) => id,
                    None => {
                        ::scale_core::Print::print("done row_bank_subsidiary_2 first");
                        ::scale_core::Print::print("\n");
                        return;
                    }
                    _ => {
                        {
                            ::scale_core::Print::print("bad format !");
                            unsafe { ::scale_core::__crash() }
                        };
                        return;
                    }
                };
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const test_name: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_name"),
            ignore: false,
            allow_fail: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(test_name())),
    };
    fn test_name() {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["prout\n"],
                &match () {
                    () => [],
                },
            ));
        };
        cosmian_std::CURRENT_TEST_NAME.with(|current_test_name| {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["fill\n"],
                    &match () {
                        () => [],
                    },
                ));
            };
            current_test_name.set("lolilol").unwrap();
        });
        lomain();
    }
}
#[main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_name])
}
