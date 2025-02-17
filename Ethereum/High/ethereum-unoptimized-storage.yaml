id: ethereum-unoptimized-storage

info:
  name: Ethereum Unoptimized Storage Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract inefficiently manages storage, leading to higher gas costs.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "balance"
          - "trait"
          - "transaction"
        condition: and
