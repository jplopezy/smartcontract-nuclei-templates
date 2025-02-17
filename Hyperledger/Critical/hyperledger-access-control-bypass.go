id: hyperledger-access-control-bypass

info:
  name: Hyperledger Access Control Bypass Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: An access control mechanism is flawed, allowing unauthorized actions.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "contract"
          - "function"
          - "mapping"
        condition: and
