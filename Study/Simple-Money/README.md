# Simple Money

## Purpose

Ease of developing appropriate implementations from the specification. A high degree of freedom should be provided in how to design programs

## Goal

Given a money minter and two purses A and B, design a transaction where user A can securely send money to user B, using the capability pattern of sealer-unsealer. 

## Architecture


![Simple Money](/images/simpleMoney.jpg)

The architecture (consisted of the main entity `mintMaker`, making the `Mint` Object, representing a new currency. It has a fixed amount of total balance. It employs the Factory Pattern to further create two `Purse` objects, Alice and Bob, which it can initialize with a certain balance. This is implemented in steps (1)-(2). Now, the question is whether Alice can pay some of its money to Bob while conserving the total currency. Some of the goals the architecture needs to achieve \citep{millerFinancial} are:

 1. (T1) Only someone with the mint has the power to change the total balance of that currency
 2. (T2) `Purse A` cannot change the balance of `Purse B`
 3. (T3) Balances should always be positive
 4. (T4) If a successful deposit gets reported, Alice should be guaranteed that the deposit was conducted in the other wallet


## Instructions

One can assume that the Sealer and Unsealer primitives, as well as the Mint object (steps (1)-(2) in the diagram), were implemented. The user tasks for both languages to securely transfer money via an intermediate Purse Object with the Sealer-Unsealer pattern. It should consist of the following methods:


- `getBalance(): Int` - Get the current balance in the purse  
- `sprout(): Purse` - Create a new empty purse 
- `getDecr(): SealedBox[Int -> void]` - Get a sealed version of `decr`. A hint was provided that should be used to validate (T4) during `deposit` to the empty Wallet and Bob. `decr` is a function that subtracts the balance in the current Purse
- `deposit(amount:Int,src:Purse):void` - Securely transmits money from one wallet to another
- `print():void` - (Optional) Print debugging information

The programmer's expected steps are to understand the respective codebase and extend the program's functionality.

The top-level code is provided in listing \ref{list:simpleMoneyTop}.

```rust
def paymentEnvelopeForBob = alicePurse.sprout()
paymentEnvelope.deposit(100, alicePurse)  
bobPurse.deposit(10, paymentEnvelope) // Assumption that Alice has access to the correct object 
... // Checks for correct balances
```

### Step 1 and Step 2 (~50 mins)

Instructions were the same for Wyvern and Rust implementation, with slightly different filenames. They were to implement the architecture above and then come up with potential vulnerabilities in their implementation. Existing boilerplate code and the todo list for implementing the function are provided in the code


### Step 3 (~10 mins)

Please provide your ratings out of 5 on the following:

1. How useful do you think capabilities are?
2. How much did you like working on Wyvern?
3. How much did you like working on Rust?
4. How much do you think you understand the concept of capabilities?

**Subjective questions**:
Is there a part of the language / task design which the participant would want to be improved?
