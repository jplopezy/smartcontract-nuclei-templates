id: ethereum-reentrancy

info:
  name: Ethereum Reentrancy Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract allows external calls before updating state, enabling reentrancy attacks.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "signer"
          - "require"
          - "transaction"
        condition: and
