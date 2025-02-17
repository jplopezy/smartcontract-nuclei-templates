id: solana-gas-limit-issues

info:
  name: Solana Gas Limit Issues Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract has loops or operations that can exceed the gas limit, leading to failed transactions.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "balance"
          - "trait"
          - "storage"
        condition: and
