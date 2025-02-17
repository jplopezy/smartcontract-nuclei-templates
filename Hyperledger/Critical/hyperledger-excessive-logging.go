id: hyperledger-excessive-logging

info:
  name: Hyperledger Excessive Logging Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract logs excessive data, increasing costs unnecessarily.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "storage"
          - "invoke"
          - "signer"
        condition: and
