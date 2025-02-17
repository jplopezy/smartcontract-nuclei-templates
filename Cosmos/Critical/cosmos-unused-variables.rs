id: cosmos-unused-variables

info:
  name: Cosmos Unused Variables Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract has unused variables, increasing contract size and potential confusion.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "dispatch"
          - "balance"
          - "key"
        condition: and
