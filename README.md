# MPC Join (Confidential Data Science)

Bob and Charlie have common customers and Alice would like to study the relationships between the amounts spent at Bob's, the category of purchases at Charlie's and the Customer Satisfaction Index recorded by both.

Bob and Charlie are interested in Alice analysis results, however neither Bob nor Charlie wants to:
 - reveal their full list of customers to the other two
 - reveal their common customer IDs to Alice
 - reveal their satisfaction index to Alice

 Alice is going to use [CipherCompute EAP](https://github.com/Cosmian/CipherCompute) to perform a confidential calculation that will secretly perform an inner join (an intersection) between Bob and Charlie customers datasets and for every matching customer row:

 - drop the customer ID before outputting the intersection to Alice
 - output the `amount spent` (from Bob's) and the `category` (from Charlie's)
 - output a composite satisfaction index calculated as: `2 x Bob's + 3 x Charlie's`

For example:

    Bob Secret Input:
        id  |  amount | sat. index
     -------|---------|------------ 
      17600 |   82721 |         83 
      21000 |   45234 |         65       

    Charlie Secret Input:
        id  |  categ. | sat. index
     -------|---------|------------ 
      13200 |       1 |         36 
      17600 |       3 |         97 

    ==> Output to Alice (match on id=17600):
    
     amount |  categ. | composite
     -------|---------|---------- 
      82721 |       3 |     457 



The code in `src/main.rs` implements this confidential collaborative computation and shows how to

 - perform a secrete inner join in O(n) using a merge join which assumes the datasets are sorted by ID ascending. Is is very easy to change this code to perform other kind of secret joins: left, right or full outer join.
 - perform some secrete arithmetics combining secret data from 2 different datasets and scalar values.


## Data Science: confidential datasets preparation

This example illustrates how to solve one of the biggest pain in performing data science on confidential data from multiple sources: generating the dataset to be analyzed in the first place!

It also illustrates how using CipherCompute makes it possible to secretly manipulate IDs (to perform a join here) then anonymize the dataset by simply dropping them before output.

A nice differential privacy touch here, would be to add some noise from a Laplace distribution on Bob's amounts before running the MPC computation.


## Hack it !

The code is heavily documented and under MIT license as it is meant to be hacked for your purpose.

It is actually very easy to generalize this code to a lot of confidential datasets generation problems.

Do not hesitate to open issues and PRs to improve the quality of this code 
and its documentation.

## Editing and testing

Once you have cloned this repository locally, edit the code; 
we recommend that you use the free VSCode and rust-analyzer extension.

To check the validity of your code, simply run  `cargo build`. 
The build process outputs [WASM](https://fr.wikipedia.org/wiki/WebAssembly) which
is what is actually fed as an intermediate representation to the CipherCompute engine.

To facilitate testing without having to run [CipherCompute EAP](https://github.com/Cosmian/CipherCompute),  2 facilities are provided via 2 scripts:

 - `emulate.sh` will pick up the input data in the `data/inputs` directory 
  and output its results in the `data/outputs` directory. These directories contain one 
  file per player. This scripts perform the same emulation as that provided on the CipherCompute UI. 

 - `test.sh` will run the unit tests of the `main.rs` file. For a test written 
   ```rust
   #[test]
    fn test_example() {
        // An example of a successful test
        // which input and expected output data are located
        // in the `fixtures/first_test` folder
        cosmian_std::test!("first_test");
        // If you change any data in the input or output files,
        // the test will fail
    }
    ```
    The input data will be picked up from the `fixtures/first_test/inputs` directory and
    the outputs will be **compared** to those of the `fixtures/first_test/outputs` directory.

## Testing inside the CipherCompute MPC engine

1. Make a change and test it using `./simulate.sh`
2. commit the change to the local git and note the git commit

3. Then use the `git-daemon.sh` script to launch a git daemon which exposes this project at
`git://localhost:9418/mpc_join`

From the UI on the CipherCompute EAP version

4. Create/update a computation using the git URL above and the git commit you want to test
5. Run the computation from the UI

See the [CipherCompute EAP](https://github.com/Cosmian/CipherCompute) Quick Start Guide
on how to use its UI to configure a computation.

