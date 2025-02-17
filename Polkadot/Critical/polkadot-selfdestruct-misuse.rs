id: polkadot-selfdestruct-misuse

info:
  name: Polkadot Selfdestruct Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract can be destroyed unexpectedly via selfdestruct, leading to asset loss.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "msg.sender"
          - "owner"
          - "key"
        condition: and
