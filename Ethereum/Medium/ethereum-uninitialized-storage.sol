id: ethereum-uninitialized-storage

info:
  name: Ethereum Uninitialized Storage Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract has uninitialized storage variables, which can be hijacked by attackers.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "trait"
          - "storage"
          - "send"
        condition: and
