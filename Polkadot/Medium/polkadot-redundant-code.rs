id: polkadot-redundant-code

info:
  name: Polkadot Redundant Code Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract contains redundant logic, making maintenance difficult.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "log"
          - "dispatch"
          - "emit"
        condition: and
