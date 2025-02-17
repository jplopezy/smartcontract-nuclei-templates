id: solana-insecure-randomness

info:
  name: Solana Insecure Randomness Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract generates random values using block attributes, which can be manipulated.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "storage"
          - "msg.sender"
          - "emit"
        condition: and
