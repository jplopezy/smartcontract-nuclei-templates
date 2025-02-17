id: hyperledger-excessive-logging

info:
  name: Hyperledger Excessive Logging Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract logs excessive data, increasing costs unnecessarily.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "log"
          - "key"
          - "transaction"
        condition: and
