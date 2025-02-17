id: solana-reentrancy

info:
  name: Solana Reentrancy Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract allows external calls before updating state, enabling reentrancy attacks.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "contract"
          - "mapping"
          - "signer"
        condition: and
