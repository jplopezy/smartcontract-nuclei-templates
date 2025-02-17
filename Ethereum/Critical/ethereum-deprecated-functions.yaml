id: ethereum-deprecated-functions

info:
  name: Ethereum Deprecated Functions Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract uses deprecated Solidity functions that may be insecure.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "require"
          - "function"
          - "balance"
        condition: and
