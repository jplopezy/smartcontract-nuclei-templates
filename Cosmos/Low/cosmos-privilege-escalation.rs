id: cosmos-privilege-escalation

info:
  name: Cosmos Privilege Escalation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract improperly grants administrative privileges.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "dispatch"
          - "msg.sender"
          - "trait"
        condition: and
