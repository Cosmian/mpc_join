#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use cosmian_std::scale::{self, println, ClearModp};
use cosmian_std::{prelude::*, Column, InputRow, OutputRow};
use scale::SecretModp;

// Players
const DATA_CONSUMER: u32 = 0;
const DATA_PROVIDER_1: u32 = 1;
const DATA_PROVIDER_2: u32 = 2;

#[cosmian_std::main(KAPPA = 40)]
#[inline(never)]
fn main() {
    println!("##### Program starting");
    loop {
        // First, fetch the first row of data provider 1 and secretly recover the id
        let mut row_dp_1 = InputRow::read(Player::<DATA_PROVIDER_1>); 
        // The id is a `SecretModp`: its value never appears in clear text in the program memory
        let mut id_dp_1 = match row_dp_1.next_col() {
            Some(Column::SecretModp(id)) => id,
            None => {
                println!("  - no more data");
                break;
            }
            _ => {
                scale::panic!("bad data format for data from provider 1!");
                return;
            }
        };

        // Then, fetch the first row of data provider 2 and secretly recover the id
        let mut row_dp_2 = InputRow::read(Player::<DATA_PROVIDER_2>); // [26, 0]
        let mut id_dp_2 = match row_dp_2.next_col() {
            Some(Column::SecretModp(id)) => id,
            None => {
                println!("  - no more data");
                break;
            }
            _ => {
                scale::panic!("bad data format for data from provider 2");
                return;
            }
        };

        // This algorithm is an inner join implemented as a merge join running in O(N)
        // that assumes that the data of both provider is sorted ascending on the id
        loop {
            // The ids match so we will output the row
            // Note the `reveal()` which will reval there is a
            // match without revealing the ids. Branching can only be performed
            // on clear text data
            if SecretI64::from(id_dp_1)
                .eq(SecretI64::from(id_dp_2))
                .reveal()
            {
                // Create the next row we are going to output to the data consumer
                let mut row_data_consumer = OutputRow::new(Player::<DATA_CONSUMER>);

                // We will "drop" the ids which we consider sensitive information
                // which may lead to identification or re-identification

                // The first value from the data provider 1 will be revealed to the data consumer
                row_data_consumer.append(
                    row_dp_1
                        .next_col()
                        .unwrap()
                        .into_secret_modp()
                        .expect("there should be a first value for DP 1"),
                );
                // Idem with the first value from the data provider 2
                row_data_consumer.append(
                    row_dp_2
                        .next_col()
                        .unwrap()
                        .into_secret_modp()
                        .expect("there should be a first value for DP 2"),
                );
                // Now let us create a composite value = 2 * v2₁ + 3 * v2₂
                let v2_dp_1 = row_dp_1
                    .next_col()
                    .unwrap()
                    .into_secret_modp()
                    .expect("there should be a second value for DP 1");
                let v2_dp_2 = row_dp_2
                    .next_col()
                    .unwrap()
                    .into_secret_modp()
                    .expect("there should be a second value for DP 2");
                // We "lift" the clear text scalar values in Zₚ using `ClearModp`
                // then perform the secret multiplication -> the result is a `SecretModP`
                let composite: SecretModp =
                    ClearModp::from(2) * v2_dp_1 + ClearModp::from(3) * v2_dp_2;
                // now reveal it to the data consumer
                row_data_consumer.append(composite);
                // that is it for this row;
                // it will be automatically flushed to the data consumer
                break;
            } else if SecretI64::from(id_dp_1)
                .lt(SecretI64::from(id_dp_2))
                .reveal()
            {
                // Fetch next row of data provider 1
                row_dp_1 = InputRow::read(Player::<DATA_PROVIDER_1>); // [16, 0]
                id_dp_1 = match row_dp_1.next_col() {
                    Some(Column::SecretModp(id)) => id,
                    None => {
                        println!("  - no more data");
                        break;
                    }
                    _ => {
                        scale::panic!("bad data format for data from provider 1!");
                        return;
                    }
                };
            } else if SecretI64::from(id_dp_2)
                .lt(SecretI64::from(id_dp_1))
                .reveal()
            {
                // Fetch next id_bank_subsidiary_2
                row_dp_2 = InputRow::read(Player::<DATA_PROVIDER_2>); // [26, 0]
                id_dp_2 = match row_dp_2.next_col() {
                    Some(Column::SecretModp(id)) => id,
                    None => {
                        println!("  - no more data");
                        break;
                    }
                    _ => {
                        scale::panic!("bad data format for data from provider 2!");
                        return;
                    }
                };
            }
        }
    }
    println!("##### Program end");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        cosmian_std::test!("first_test");
    }

    #[test]
    fn test_second() {
        cosmian_std::test!("another_one");
    }
}
