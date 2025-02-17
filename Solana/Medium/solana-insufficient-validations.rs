id: solana-insufficient-validations

info:
  name: Solana Insufficient Validations Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract does not validate inputs properly, leading to potential exploits.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "storage"
          - "balance"
          - "emit"
        condition: and
