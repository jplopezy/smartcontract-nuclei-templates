id: polkadot-gas-limit-issues

info:
  name: Polkadot Gas Limit Issues Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract has loops or operations that can exceed the gas limit, leading to failed transactions.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "send"
          - "signer"
          - "dispatch"
        condition: and
