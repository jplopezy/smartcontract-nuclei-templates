id: solana-selfdestruct-misuse

info:
  name: Solana Selfdestruct Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract can be destroyed unexpectedly via selfdestruct, leading to asset loss.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "emit"
          - "key"
          - "function"
        condition: and
