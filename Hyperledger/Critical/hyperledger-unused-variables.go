id: hyperledger-unused-variables

info:
  name: Hyperledger Unused Variables Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract has unused variables, increasing contract size and potential confusion.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "dispatch"
          - "invoke"
          - "function"
        condition: and
