id: cosmos-integer-overflow

info:
  name: Cosmos Integer Overflow Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract performs arithmetic operations without checking for overflows or underflows.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "msg.sender"
          - "balance"
          - "unchecked"
        condition: and
