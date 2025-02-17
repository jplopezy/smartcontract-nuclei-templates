id: cosmos-reentrancy

info:
  name: Cosmos Reentrancy Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract allows external calls before updating state, enabling reentrancy attacks.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "invoke"
          - "signer"
          - "balance"
        condition: and
