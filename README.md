# Nuclei Smart Contract Source Review

This repository collects **heuristic file-protocol patterns** for reviewing Solidity, Solana Rust and other smart-contract-related source code. It is not a smart-contract auditor or a substitute for Slither, semgrep, Anchor-specific analysis, manual review, or dynamic tests.

## Verified additions (2026)

The new `Ethereum/Review/` and `Solana/Review/` templates are deliberately marked **info**. A match means **inspect this code**, not that the code is exploitable.

- `ethereum-tx-origin-authorization-review` — explicit `tx.origin` comparisons in Solidity `require` statements.
- `ethereum-ether-call-before-state-write-review` — candidate value transfer before a nearby balance write.
- `ethereum-discarded-low-level-call-review` — discarded `.call`, `.delegatecall`, or `.staticcall` results on a standalone statement.
- `solana-unchecked-account-data-review` — explicit unchecked deserialization calls in Rust.

The repository's older severity-labeled templates are *legacy unvalidated regex heuristics* and may be overly broad, incomplete or contain regex/YAML syntax defects. **Do not treat their assigned severities as confirmed vulnerabilities.** Their labels require a separate audit before production use.

## Usage

Install ProjectDiscovery Nuclei, then validate before scanning:

```sh
nuclei -validate -t Ethereum/Review/ -t Solana/Review/
```

For a repository or directory of **your own or authorized code**, use the file protocol:

```sh
nuclei -t Ethereum/Review/ -t Solana/Review/ -target ./authorized-contracts/
```

The `file` protocol operates on local files and filters by `.sol` and `.rs`. A Solidity filename is a local path, not an HTTP URL. Avoid passing secrets or private client code to external services.

## Offline pattern tests

```sh
python3 -m pip install pyyaml
python3 -m unittest discover -s tests -v
```

These tests validate YAML structure and Python regular-expression behavior on synthetic positives and negatives; they are **not a substitute** for `nuclei -validate` and an integration run with a supported Nuclei version.

## Authoring guidelines

- Quote regex patterns with **single-quoted YAML strings** to avoid invalid `\\` escapes.
- Use extensions with leading periods, e.g. `.sol` or `.rs`.
- Use `info` for syntactic indicators and raise severity only after evidence supports impact.
- Include both positive and negative fixtures to reduce false positives.
- Do not label mere usage of a security API as proof of missing authorization checks.
