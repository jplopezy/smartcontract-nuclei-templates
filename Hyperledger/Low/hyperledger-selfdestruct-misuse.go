id: hyperledger-selfdestruct-misuse

info:
  name: Hyperledger Selfdestruct Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract can be destroyed unexpectedly via selfdestruct, leading to asset loss.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "contract"
          - "balance"
          - "emit"
        condition: and
