id: hyperledger-gas-limit-issues

info:
  name: Hyperledger Gas Limit Issues Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract has loops or operations that can exceed the gas limit, leading to failed transactions.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "require"
          - "mapping"
          - "send"
        condition: and
