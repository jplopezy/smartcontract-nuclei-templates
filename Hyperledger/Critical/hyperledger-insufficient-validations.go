id: hyperledger-insufficient-validations

info:
  name: Hyperledger Insufficient Validations Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract does not validate inputs properly, leading to potential exploits.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "key"
          - "log"
          - "trait"
        condition: and
