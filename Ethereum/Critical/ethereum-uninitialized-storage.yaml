id: ethereum-uninitialized-storage

info:
  name: Ethereum Uninitialized Storage Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract has uninitialized storage variables, which can be hijacked by attackers.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "function"
          - "balance"
          - "transaction"
        condition: and
