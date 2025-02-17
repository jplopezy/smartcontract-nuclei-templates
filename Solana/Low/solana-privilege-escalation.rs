id: solana-privilege-escalation

info:
  name: Solana Privilege Escalation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract improperly grants administrative privileges.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "balance"
          - "storage"
          - "dispatch"
        condition: and
