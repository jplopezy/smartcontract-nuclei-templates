id: ethereum-insufficient-validations

info:
  name: Ethereum Insufficient Validations Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract does not validate inputs properly, leading to potential exploits.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "function"
          - "storage"
          - "transaction"
        condition: and
