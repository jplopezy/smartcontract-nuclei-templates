id: cosmos-deprecated-functions

info:
  name: Cosmos Deprecated Functions Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract uses deprecated Solidity functions that may be insecure.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "trait"
          - "storage"
          - "transaction"
        condition: and
