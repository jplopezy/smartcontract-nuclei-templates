id: ethereum-privilege-escalation

info:
  name: Ethereum Privilege Escalation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract improperly grants administrative privileges.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "dispatch"
          - "function"
          - "trait"
        condition: and
