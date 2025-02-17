id: polkadot-privilege-escalation

info:
  name: Polkadot Privilege Escalation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract improperly grants administrative privileges.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "storage"
          - "dispatch"
          - "invoke"
        condition: and
