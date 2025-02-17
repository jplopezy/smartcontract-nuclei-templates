id: cosmos-insecure-randomness

info:
  name: Cosmos Insecure Randomness Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract generates random values using block attributes, which can be manipulated.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "transaction"
          - "dispatch"
          - "msg.sender"
        condition: and
