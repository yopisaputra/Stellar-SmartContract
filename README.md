# Soroban Crowdfunding Smart Contract

This is an example implementation of a crowdfunding smart contract built with Rust and Soroban to run on the Stellar blockchain.

This contract allows a creator to launch a fundraising campaign with a specific goal and deadline. Donors can send XLM tokens to the contract. If the goal is not met by the deadline, donors can withdraw their funds.

## 🚀 Key Features
- **Campaign Initialization**: Set a creator, funding goal, and deadline.
- **Donations**: Accept donations in the form of XLM tokens.
- **Progress Tracking**: View the total amount raised and its percentage of the goal.
- **Refunds**: Allow donors to withdraw their funds if the goal is not met after the deadline.
- **Status Checks**: Functions to view the campaign status (has it ended, has the goal been reached).

---

## 🛠️ 1. Environment Setup (Prerequisites)

Before you begin, ensure you have the following tools installed.

### a. Rust Installation
If you don't have it installed, follow the instructions on the official site:
➡️ **[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)**

### b. Soroban CLI Installation
This is the primary tool for building, deploying, and interacting with Soroban smart contracts.

```sh
cargo install soroban-cli
```

After installation, **close and reopen your terminal**, then verify it works by running:
```sh
soroban --version
```
You should see a version number like `soroban-cli 20.x.x`.

<details>
<summary>⚠️ **Troubleshooting Soroban CLI Installation**</summary>

If you encounter issues during installation, try these solutions:

1.  **Error: `soroban: command not found`**
    This means the `cargo` installation directory is not in your terminal's `PATH`. Run this command, then close and reopen your terminal.
    ```sh
    source "$HOME/.cargo/env"
    ```

2.  **Error: `binary 'stellar' already exists...`**
    This indicates a conflict with an older version (`stellar-cli`). Use the `--force` flag to overwrite it.
    ```sh
    cargo install soroban-cli --force
    ```

3.  **Compilation Error (`could not compile soroban-cli`)**
    This can happen due to an incompatible cache or version mismatch. The solution is a clean installation:
    ```sh
    # 1. Uninstall any partially installed versions
    cargo uninstall soroban-cli

    # 2. Try installing again
    cargo install soroban-cli
    ```
</details>

---

## ⚙️ 2. Project Workflow

### a. Clone the Repository
```sh
# Replace with your repository URL if needed
git clone https://github.com/your-username/yopi-token.git
cd yopi-token/contracts/crowdfunding
```

### b. Run Local Tests
Before deploying, ensure all contract logic works as expected by running the local tests.
```sh
cargo test
```

### c. Build the Contract
Compile the Rust code into a WebAssembly (.wasm) file ready for deployment.
```sh
soroban contract build
```
The output file will be located at `../../target/wasm32-unknown-unknown/release/crowdfunding.wasm`.

---

## 🌐 3. Tutorial: Deploy & Interact on Testnet

This tutorial shows how to deploy and interact with the contract on the **Stellar Testnet**.

### a. Set Up a Testnet Account
You need a Testnet account with a balance of XLM to pay for transaction fees.

```sh
# Create a new identity named 'user1' (the name can be anything)
soroban config identity generate user1

# Get its public address (starts with 'G...')
soroban config identity address user1

# Request funds from the Testnet Friendbot. Open the link below in a browser
# and paste your 'G...' address to get free XLM.
# -> https://friendbot.stellar.org/
```

### b. Deploy the Contract
Deploy the `.wasm` file you built to the Testnet.

```sh
soroban contract deploy \
  --wasm ../../target/wasm32-unknown-unknown/release/crowdfunding.wasm \
  --source user1 \
  --network testnet
```
**IMPORTANT**: Copy the **Contract ID** (starts with `C...`) that appears after a successful deployment. Save this ID.

### c. Initialize the Campaign
Once deployed, the contract must be initialized.

- **`--id`**: The Contract ID from the previous step.
- **`--owner`**: The address that will own the campaign (use your `user1` address).
- **`--goal`**: The funding target in stroops (1 XLM = 10,000,000 stroops). Example: 100 XLM = `1000000000`.
- **`--deadline`**: The deadline as a Unix timestamp.
- **`--xlm_token`**: The address of the XLM token contract on Testnet (this is a fixed address).

```sh
# Replace <YOUR_CONTRACT_ID> and <YOUR_ACCOUNT_ADDRESS>
# Example deadline: 24 hours from now
DEADLINE=$(($(date +%s) + 86400))
CONTRACT_ID="<YOUR_CONTRACT_ID>"
OWNER_ADDRESS="<YOUR_ACCOUNT_ADDRESS>"
XLM_TOKEN_ADDRESS="CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"

soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source user1 \
  --network testnet \
  --fn initialize \
  --arg "{\\"owner\\":\\"$OWNER_ADDRESS\\",\\"goal\\":1000000000,\\"deadline\\":$DEADLINE,\\"xlm_token\\":\\"$XLM_TOKEN_ADDRESS\\"}"
```

### d. Donate to the Campaign
Now, anyone can donate. Here, we use `user1` as the donor.

```sh
# Example donation: 20 XLM = 200000000 stroops
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source user1 \
  --network testnet \
  --fn donate \
  --arg "{\\"donor\\":\\"$OWNER_ADDRESS\\",\\"amount\\":200000000}"
```

### e. Check Campaign Status
Use read-only functions to view progress without sending a transaction.

```sh
# Check the total amount raised
soroban contract read --id "$CONTRACT_ID" --fn get_total_raised

# Check the progress percentage
soroban contract read --id "$CONTRACT_ID" --fn get_progress_percentage
```

### f. Withdraw Funds (Refund)
If the deadline has passed AND the goal was not met, a donor can request a refund.

```sh
# Make sure you run this after the deadline has passed
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source user1 \
  --network testnet \
  --fn refund \
  --arg "{\\"donor\\":\\"$OWNER_ADDRESS\\"}"
```
