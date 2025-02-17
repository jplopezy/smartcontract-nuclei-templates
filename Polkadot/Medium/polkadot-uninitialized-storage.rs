id: polkadot-uninitialized-storage

info:
  name: Polkadot Uninitialized Storage Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract has uninitialized storage variables, which can be hijacked by attackers.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "log"
          - "dispatch"
          - "function"
        condition: and
