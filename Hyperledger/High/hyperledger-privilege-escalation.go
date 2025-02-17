id: hyperledger-privilege-escalation

info:
  name: Hyperledger Privilege Escalation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract improperly grants administrative privileges.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "dispatch"
          - "owner"
          - "send"
        condition: and
