id: cosmos-reentrancy

info:
  name: Cosmos Reentrancy Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract allows external calls before updating state, enabling reentrancy attacks.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "owner"
          - "emit"
          - "mapping"
        condition: and
