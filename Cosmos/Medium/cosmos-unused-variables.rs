id: cosmos-unused-variables

info:
  name: Cosmos Unused Variables Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract has unused variables, increasing contract size and potential confusion.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "contract"
          - "msg.sender"
          - "log"
        condition: and
