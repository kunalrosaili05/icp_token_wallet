# `ICP Token Wallet`

This repository contains the code for a Rust-based smart contract deployed on the Internet Computer (ICP) testnet. The smart contract includes features for sending tokens, receiving tokens, and checking balances. The project is set up and deployed using Windows Subsystem for Linux (WSL).


## Prerequisites

WSL (Windows Subsystem for Linux): Ensure WSL is installed and set up on your Windows system.
Rust: Install Rust version 1.81.0 or above.

    Use Rustup for installation: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh



DFX: Install DFINITY's SDK version 0.24.0.

    Installation command: sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"




### Setting Up the Environment

Navigate to your project folder:

cd /path/to/your/project

verify Rust and DFX versions:

rustc --version
dfx --version

Start the DFX local network (for local testing):

dfx start

Build the project:

dfx build

Deploy the project to the testnet:

dfx deploy


## Features

Send Tokens: Allows users to transfer tokens to other addresses.
Receive Tokens: Updates wallet balance when tokens are received.
Check Balances: Query the current token balance for any address.


##  Notes

Make sure to configure the .dfx and environment variables appropriately for deployment.
This project requires the ICP testnet for testing and deployment. Ensure you have an Internet Computer Principal ID.
