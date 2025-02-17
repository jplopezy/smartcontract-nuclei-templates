id: ethereum-selfdestruct-misuse

info:
  name: Ethereum Selfdestruct Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract can be destroyed unexpectedly via selfdestruct, leading to asset loss.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "dispatch"
          - "balance"
          - "invoke"
        condition: and
