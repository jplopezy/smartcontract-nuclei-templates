id: ethereum-integer-overflow

info:
  name: Ethereum Integer Overflow Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract performs arithmetic operations without checking for overflows or underflows.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "mapping"
          - "msg.sender"
          - "invoke"
        condition: and
