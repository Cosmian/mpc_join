#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use cosmian_std::scale::{self, println};
use cosmian_std::{prelude::*, Column, InputRow, OutputRow};

// Players
const BANK_MORTGAGE: u32 = 0;
const BANK_SUBSIDIARY1: u32 = 1;
const BANK_SUBSIDIARY2: u32 = 2;

#[cosmian_std::main(KAPPA = 40)]
#[inline(never)]
fn main() {
    println!("##### Reading from players");

    loop {
        // First fetch of id player 1
        let mut row_bank_subsidiary_1 = InputRow::read(Player::<BANK_SUBSIDIARY1>); // [26, 0]
        let mut id_bank_subsidiary_1 = match row_bank_subsidiary_1.next_col() {
            Some(Column::SecretModp(id)) => id,
            None => {
                println!("done row_bank_subsidiary_1 first");
                return;
            }
            _ => {
                scale::panic!("bad format !");
                return;
            }
        };

        // First fetch of id player 2
        let mut row_bank_subsidiary_2 = InputRow::read(Player::<BANK_SUBSIDIARY2>); // [26, 0]
        let mut id_bank_subsidiary_2 = match row_bank_subsidiary_2.next_col() {
            Some(Column::SecretModp(id)) => id,
            None => {
                println!("done row_bank_subsidiary_2 first");
                return;
            }
            _ => {
                scale::panic!("bad format !");
                return;
            }
        };

        loop {
            if SecretI64::from(id_bank_subsidiary_1)
                .eq(SecretI64::from(id_bank_subsidiary_2))
                .reveal()
            {
                let mut row_bank_mortgage = OutputRow::new(Player::<BANK_MORTGAGE>);

                // Add columns in the row for BANK_MORTGAGE [id, p1_col1, p1_col2, p2_col1, p2_col2]
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

                break; // It's good because we need to fetch new id_bank_subsidiary_1 and new id_bank_subsidiary_2
            } else if SecretI64::from(id_bank_subsidiary_1)
                .lt(SecretI64::from(id_bank_subsidiary_2))
                .reveal()
            {
                // Fetch next id_bank_subsidiary_1
                row_bank_subsidiary_1 = InputRow::read(Player::<BANK_SUBSIDIARY1>); // [16, 0]
                id_bank_subsidiary_1 = match row_bank_subsidiary_1.next_col() {
                    Some(Column::SecretModp(id)) => id,
                    None => {
                        println!("done row_bank_subsidiary_1 first");
                        return;
                    }
                    _ => {
                        scale::panic!("bad format !");
                        return;
                    }
                };
            } else if SecretI64::from(id_bank_subsidiary_2)
                .lt(SecretI64::from(id_bank_subsidiary_1))
                .reveal()
            {
                // Fetch next id_bank_subsidiary_2
                row_bank_subsidiary_2 = InputRow::read(Player::<BANK_SUBSIDIARY2>); // [26, 0]
                id_bank_subsidiary_2 = match row_bank_subsidiary_2.next_col() {
                    Some(Column::SecretModp(id)) => id,
                    None => {
                        println!("done row_bank_subsidiary_2 first");
                        return;
                    }
                    _ => {
                        scale::panic!("bad format !");
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

    #[test]
    fn test_name() {
        cosmian_std::test!("lolilol");
    }

    #[test]
    fn test_second() {
        cosmian_std::test!("another_one");
    }
}
