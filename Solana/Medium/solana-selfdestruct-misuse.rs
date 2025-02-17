id: solana-selfdestruct-misuse

info:
  name: Solana Selfdestruct Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract can be destroyed unexpectedly via selfdestruct, leading to asset loss.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "contract"
          - "mapping"
          - "function"
        condition: and
