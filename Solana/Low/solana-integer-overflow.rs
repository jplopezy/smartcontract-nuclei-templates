id: solana-integer-overflow

info:
  name: Solana Integer Overflow Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract performs arithmetic operations without checking for overflows or underflows.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "trait"
          - "emit"
          - "key"
        condition: and
