id: solana-reentrancy

info:
  name: Solana Reentrancy Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract allows external calls before updating state, enabling reentrancy attacks.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "contract"
          - "function"
          - "log"
        condition: and
