id: hyperledger-excessive-logging

info:
  name: Hyperledger Excessive Logging Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract logs excessive data, increasing costs unnecessarily.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "balance"
          - "unchecked"
          - "transaction"
        condition: and
