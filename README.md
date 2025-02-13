# Minimal NFT Token2022 Example

This repository provides a minimalistic example that highlights the differences between the SPL Token program and the Token2022 program when minting NFTs and creating associated Metadata accounts using Metaplex.

## Overview

This project contains a Rust program written with [Anchor](https://github.com/project-serum/anchor) that mints an NFT and creates its Metadata account. Two TypeScript test cases are provided:

- **SPL Token Program Test:**  
  Running the test with the standard SPL Token program successfully mints the NFT and creates its Metadata account.

- **Token2022 Program Test:**  
  Running the test with the Token2022 program fails during the Metadata creation step with the error: 
**"_Program log: Instruction not supported for ProgrammableNonFungible assets._"**
