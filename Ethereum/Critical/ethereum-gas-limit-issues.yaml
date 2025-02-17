id: ethereum-gas-limit-issues

info:
  name: Ethereum Gas Limit Issues Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract has loops or operations that can exceed the gas limit, leading to failed transactions.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "function"
          - "require"
          - "owner"
        condition: and
