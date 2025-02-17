id: hyperledger-deprecated-functions

info:
  name: Hyperledger Deprecated Functions Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract uses deprecated Solidity functions that may be insecure.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "log"
          - "balance"
          - "require"
        condition: and
