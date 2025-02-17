id: solana-unused-variables

info:
  name: Solana Unused Variables Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract has unused variables, increasing contract size and potential confusion.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "function"
          - "mapping"
          - "transaction"
        condition: and
