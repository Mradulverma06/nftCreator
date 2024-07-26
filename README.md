NFT Creator Contract

This project implements a simple NFT (Non-Fungible Token) creator contract using the Soroban SDK. The contract allows users to create and retrieve NFTs, each associated with a unique identifier and metadata stored on the blockchain.

Overview

The 'Nftcreator' contract provides two main functions:

1. `createnft`: Creates a new NFT with the specified owner and IPFS metadata.
2. `getnft`: Retrieves the details of an NFT by its unique identifier.

Features

- **Create NFT**: Allows users to mint new NFTs with ownership and IPFS metadata.
- **Retrieve NFT**: Fetches the details of a specific NFT by its ID.

Contract Details

Data Structures

- `DigiNft`: Represents the NFT with its ID, owner, and IPFS metadata.
- `Mapnft`: Enum to handle NFT storage with the `DigiNft` variant.
- `COUNT`: A symbol used to keep track of the total number of NFTs created.

Functions

- `createnft(env: Env, owr: String, ipfs: String) -> u64`
  - **Parameters**:
    - `owr`: The owner of the NFT.
    - `ipfs`: The IPFS metadata link.
  - **Returns**: The ID of the newly created NFT.
  - **Description**: Increments the NFT count, creates a new `DigiNft` instance, stores it in the blockchain, and returns the new NFT's ID.

- `getnft(env: Env, id: u64) -> DigiNft`
  - **Parameters**:
    - `id`: The ID of the NFT to retrieve.
  - **Returns**: The `DigiNft` instance associated with the given ID.
  - **Description**: Fetches the NFT details from the blockchain. If the NFT does not exist, it returns a default `DigiNft` with "Invalid" values.

