id: hyperledger-selfdestruct-misuse

info:
  name: Hyperledger Selfdestruct Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract can be destroyed unexpectedly via selfdestruct, leading to asset loss.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "invoke"
          - "storage"
          - "function"
        condition: and
