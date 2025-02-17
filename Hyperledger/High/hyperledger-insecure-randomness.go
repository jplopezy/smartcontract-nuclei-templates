id: hyperledger-insecure-randomness

info:
  name: Hyperledger Insecure Randomness Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract generates random values using block attributes, which can be manipulated.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "log"
          - "trait"
          - "owner"
        condition: and
