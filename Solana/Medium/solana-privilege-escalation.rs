id: solana-privilege-escalation

info:
  name: Solana Privilege Escalation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract improperly grants administrative privileges.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "storage"
          - "trait"
          - "runtime_upgrade"
        condition: and
