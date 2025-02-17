id: solana-deprecated-functions

info:
  name: Solana Deprecated Functions Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract uses deprecated Solidity functions that may be insecure.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "storage"
          - "trait"
          - "dispatch"
        condition: and
