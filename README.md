# Counter Canister

This is a simple counter canister implemented in Rust for the Internet Computer (IC). The canister provides basic functionality to get, set, and increment a counter value. It uses the `candid` library for data serialization and `ic-cdk` for interacting with the IC.

---

## Features

- **Get the current counter value**: Query the current value of the counter.
- **Set the counter value**: Update the counter to a specific value.
- **Increment the counter**: Increase the counter value by 1.

---

## Prerequisites

Before running the project, ensure you have the following installed:

1. **DFX (Internet Computer SDK)**:
   - Install DFX by following the official guide: [DFX Installation](https://smartcontracts.org/docs/developers-guide/install-upgrade-remove.html).

2. **Rust**:
   - Install Rust using `rustup`: [Rust Installation](https://www.rust-lang.org/tools/install).

3. **Cargo**:
   - Cargo is installed automatically with Rust.

4. **Node.js** (optional, for frontend development):
   - Install Node.js from [Node.js Official Website](https://nodejs.org/).

---

## Running the Project

### Step 1: Clone the Repository
Clone the repository and navigate to the project directory:
```bash
git clone https://github.com/0x15ba88ff/w3ic-counter.git
cd w3ic-counter
```

### Step 2: Install Dependencies
Ensure all dependencies are installed:
```bash
cargo update
```

### Step 3: Build the Canister
Use the provided `Makefile` to build the canister:
```bash
make build
```

### Step 4: Deploy the Canister
Deploy the canister to the local IC replica:
```bash
make deploy
```

### Step 5: Interact with the Canister
Use the following commands to interact with the canister:

1. **Get the current counter value**:
   ```bash
   dfx canister call counter get
   ```

2. **Set the counter value**:
   ```bash
   dfx canister call counter set '(42)'
   ```

3. **Increment the counter**:
   ```bash
   dfx canister call counter inc
   ```
