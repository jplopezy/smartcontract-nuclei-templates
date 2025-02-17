id: hyperledger-integer-overflow

info:
  name: Hyperledger Integer Overflow Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract performs arithmetic operations without checking for overflows or underflows.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "emit"
          - "dispatch"
          - "require"
        condition: and
