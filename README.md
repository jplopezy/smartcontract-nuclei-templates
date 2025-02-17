# Smart Contract Security Templates for Nuclei

This repository provides **extensive security scanning templates** for **smart contracts** using **Nuclei**. Inspired by [Optiv's Mobile Nuclei Templates](https://github.com/optiv/mobile-nuclei-templates), these templates are designed to identify vulnerabilities in **Ethereum, Solana, Hyperledger, Cosmos, and Polkadot** smart contracts.

## 🛠 How to Use

To execute a scan, **Nuclei** should be run with the following command:

```sh
nuclei.exe -t templates/ -u test.sol -file
```

- `-t templates/` ensures that all the templates in the directory are used.
- `-u test.sol` specifies the target smart contract file.
- `-file` enables scanning of **local files** instead of URLs.

## 🔍 What Is Detected?

The templates cover a **wide range of smart contract vulnerabilities**, categorized as follows:

- 🛑 **Critical Issues**: Reentrancy attacks, Privilege escalation, Delegatecall misuse, Access control bypass.
- ⚠️ **High Severity**: Integer overflow, Insecure randomness, Unchecked external calls, Weak authentication.
- 🟡 **Medium Severity**: Gas limit issues, Event emission errors, Uninitialized storage.
- 🔵 **Low Severity**: Unused variables, Deprecated functions, Unoptimized storage.

Each template explicitly defines the **blockchain-specific file extensions**:

| **Blockchain** | **Language** | **Extension** |
|--------------|------------|-------------|
| **Ethereum** | Solidity | `.sol` |
| **Binance Smart Chain** | Solidity | `.sol` |
| **Solana** | Rust | `.rs` |
| **Hyperledger Fabric** | Go / JavaScript | `.go` / `.js` |
| **Cosmos SDK** | Go / Rust | `.go` / `.rs` |
| **Polkadot (Substrate)** | Rust | `.rs` |
| **Tezos** | Michelson | `.tz` |
| **Cardano** | Plutus (Haskell) | `.hs` |

## 📌 Important Disclaimer

These templates **are designed to assist security researchers and developers** in identifying vulnerabilities. However:

- **False positives may occur**, and manual validation is always recommended.
- **Security findings should be reviewed carefully** before reporting or remediating issues.
- If **an error or unexpected behavior** is encountered, feedback is greatly appreciated.

## 🛠 Additional Features & Improvements

- ✅ **Regex-based matchers** to reduce false positives.
- ✅ **Optimized file structure** with organized categories by severity and blockchain.
- ✅ **Custom tags** to allow selective execution of templates.
- ✅ **Scripts for automation**:
  - `update.sh` → Automatically updates Nuclei templates.
  - `run_tests.sh` → Runs scans against known vulnerable contracts.

## 💬 Feedback & Contributions

- If a **bug, false positive, or an issue with detection** is found, please let us know.
- Contributions to improve accuracy or expand the templates are always welcome.

🔒 **Stay secure and happy testing!**

