id: polkadot-insecure-randomness

info:
  name: Polkadot Insecure Randomness Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract generates random values using block attributes, which can be manipulated.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "contract"
          - "require"
          - "trait"
        condition: and
