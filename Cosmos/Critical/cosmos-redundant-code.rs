id: cosmos-redundant-code

info:
  name: Cosmos Redundant Code Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract contains redundant logic, making maintenance difficult.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "function"
          - "transaction"
          - "owner"
        condition: and
