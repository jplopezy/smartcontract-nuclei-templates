id: hyperledger-integer-overflow

info:
  name: Hyperledger Integer Overflow Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract performs arithmetic operations without checking for overflows or underflows.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "mapping"
          - "dispatch"
          - "invoke"
        condition: and
