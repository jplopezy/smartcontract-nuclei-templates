id: polkadot-integer-overflow

info:
  name: Polkadot Integer Overflow Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract performs arithmetic operations without checking for overflows or underflows.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "storage"
          - "owner"
          - "require"
        condition: and
