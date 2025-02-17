id: polkadot-deprecated-functions

info:
  name: Polkadot Deprecated Functions Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract uses deprecated Solidity functions that may be insecure.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "unchecked"
          - "emit"
          - "invoke"
        condition: and
