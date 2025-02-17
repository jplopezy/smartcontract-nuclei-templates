id: hyperledger-privilege-escalation

info:
  name: Hyperledger Privilege Escalation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract improperly grants administrative privileges.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "dispatch"
          - "transaction"
          - "function"
        condition: and
