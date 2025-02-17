id: cosmos-selfdestruct-misuse

info:
  name: Cosmos Selfdestruct Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract can be destroyed unexpectedly via selfdestruct, leading to asset loss.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "msg.sender"
          - "invoke"
          - "storage"
        condition: and
