id: ethereum-insecure-randomness

info:
  name: Ethereum Insecure Randomness Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract generates random values using block attributes, which can be manipulated.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "contract"
          - "key"
          - "runtime_upgrade"
        condition: and
