id: hyperledger-access-control-bypass

info:
  name: Hyperledger Access Control Bypass Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: An access control mechanism is flawed, allowing unauthorized actions.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "contract"
          - "function"
          - "runtime_upgrade"
        condition: and
