id: ethereum-redundant-code

info:
  name: Ethereum Redundant Code Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract contains redundant logic, making maintenance difficult.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "unchecked"
          - "transaction"
          - "dispatch"
        condition: and
