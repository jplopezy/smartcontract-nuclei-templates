id: ethereum-insecure-randomness

info:
  name: Ethereum Insecure Randomness Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract generates random values using block attributes, which can be manipulated.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "storage"
          - "emit"
          - "owner"
        condition: and
