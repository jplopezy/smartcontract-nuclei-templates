id: polkadot-privilege-escalation

info:
  name: Polkadot Privilege Escalation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract improperly grants administrative privileges.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "send"
          - "log"
          - "storage"
        condition: and
