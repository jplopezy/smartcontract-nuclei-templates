id: hyperledger-unused-variables

info:
  name: Hyperledger Unused Variables Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract has unused variables, increasing contract size and potential confusion.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "key"
          - "send"
          - "trait"
        condition: and
