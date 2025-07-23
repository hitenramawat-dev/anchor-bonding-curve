# Bonding Curve Token

A Solana program and scripts for deploying and interacting with a bonding curve token, using Anchor framework.

**Bonding curve inspired by [pump.fun](https://pump.fun/). The curve equation used is:**

```
(x + OFFSET) × (y + K1) = K2
```

Where `x` is the number of tokens, and `y`, `OFFSET`, `K1`, `K2` are curve parameters.

## Features
- Deploy a bonding curve token smart contract
- Initialize the bonding curve
- Add metadata to the token
- Buy and sell tokens via scripts

## Prerequisites
- Node.js & npm/yarn
- Rust & Solana CLI
- Anchor (for Solana smart contract development)

## Setup
1. Clone the repository:
   ```bash
   git clone <repo-url>
   cd bonding-curve-token
   ```
2. Install dependencies:
   ```bash
   npm install
   # or
   yarn install
   ```
3. Build the Solana program:
   ```bash
   anchor build
   ```

## Running Scripts
Scripts are located in `programs/bonding-curve-token/scripts/`.

- **Initialize Bonding Curve:**
  ```bash
  ts-node programs/bonding-curve-token/scripts/init_curve.ts
  ```
- **Buy Tokens:**
  ```bash
  ts-node programs/bonding-curve-token/scripts/buy_tokens.ts
  ```
- **Sell Tokens:**
  ```bash
  ts-node programs/bonding-curve-token/scripts/sell_tokens.ts
  ```
- **Add Metadata:**
  ```bash
  ts-node programs/bonding-curve-token/scripts/add_metadata.ts
  ```

## Testing
Run tests with:
```bash
anchor test
```

## Notes
- Update your `.env` file with the correct Solana cluster and wallet settings.
- Make sure you have SOL in your wallet for transactions.

---
Simple bonding curve token project for Solana, using Anchor and TypeScript scripts.