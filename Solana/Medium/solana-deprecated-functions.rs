id: solana-deprecated-functions

info:
  name: Solana Deprecated Functions Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract uses deprecated Solidity functions that may be insecure.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "contract"
          - "trait"
          - "emit"
        condition: and
