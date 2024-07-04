/*
Exercise
Let's create an empty instantiate function. Here are the steps we're interested in:

Write a use declaration to get access to the entry_point attribute from the CW library.
Write the signature for a public Rust function called instantiate. 
For now, we only want to write the function with an empty parameters list, 
and empty body and no return, like this pub fn my_function() {}. 
We'll fill in the rest as we go along.

*/

use cosmwasm_std::entry_point;

#[entry_point]

pub fn instantiate() {}



/*
In Rust , there is the concept of an "entry point". 
This is typically called main and is the first function called when a compiled application is executed.

There is no main function in CosmWasm.
We instruct the code of that fact with a function attribute, using entry_point from the cosmwasm_std library:
Instantiate is one of three such entry points into a smart contract.
It's used during deployment to initialize settings for a contract that's been stored on-chain, 
and create an address on the blockchain where the contract can be called. 

Maybe you recognize this pattern, as it's called a Constructor in many programming languages.

*/
