id: ethereum-unused-variables

info:
  name: Ethereum Unused Variables Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract has unused variables, increasing contract size and potential confusion.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "balance"
          - "owner"
          - "unchecked"
        condition: and
