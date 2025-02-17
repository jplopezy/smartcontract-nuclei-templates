id: hyperledger-redundant-code

info:
  name: Hyperledger Redundant Code Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract contains redundant logic, making maintenance difficult.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "function"
          - "log"
          - "mapping"
        condition: and
