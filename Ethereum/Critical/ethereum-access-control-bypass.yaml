id: ethereum-access-control-bypass

info:
  name: Ethereum Access Control Bypass Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: An access control mechanism is flawed, allowing unauthorized actions.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "log"
          - "require"
          - "transaction"
        condition: and
