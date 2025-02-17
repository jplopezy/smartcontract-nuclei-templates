id: polkadot-reentrancy

info:
  name: Polkadot Reentrancy Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract allows external calls before updating state, enabling reentrancy attacks.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "require"
          - "unchecked"
          - "signer"
        condition: and
