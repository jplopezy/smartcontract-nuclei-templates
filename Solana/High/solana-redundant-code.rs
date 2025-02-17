id: solana-redundant-code

info:
  name: Solana Redundant Code Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract contains redundant logic, making maintenance difficult.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "key"
          - "invoke"
          - "emit"
        condition: and
