id: hyperledger-privilege-escalation

info:
  name: Hyperledger Privilege Escalation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract improperly grants administrative privileges.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "trait"
          - "function"
          - "log"
        condition: and
