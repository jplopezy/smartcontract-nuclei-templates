id: solana-insufficient-validations

info:
  name: Solana Insufficient Validations Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract does not validate inputs properly, leading to potential exploits.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "msg.sender"
          - "storage"
          - "function"
        condition: and
