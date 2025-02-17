id: solana-selfdestruct-misuse

info:
  name: Solana Selfdestruct Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract can be destroyed unexpectedly via selfdestruct, leading to asset loss.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "owner"
          - "emit"
          - "unchecked"
        condition: and
