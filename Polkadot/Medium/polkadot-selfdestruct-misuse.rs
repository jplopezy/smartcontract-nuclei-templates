id: polkadot-selfdestruct-misuse

info:
  name: Polkadot Selfdestruct Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract can be destroyed unexpectedly via selfdestruct, leading to asset loss.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "owner"
          - "invoke"
          - "emit"
        condition: and
