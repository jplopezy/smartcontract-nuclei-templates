id: hyperledger-privilege-escalation

info:
  name: Hyperledger Privilege Escalation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract improperly grants administrative privileges.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "runtime_upgrade"
          - "function"
          - "log"
        condition: and
