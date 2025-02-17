id: ethereum-access-control-bypass

info:
  name: Ethereum Access Control Bypass Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: An access control mechanism is flawed, allowing unauthorized actions.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "transaction"
          - "send"
          - "log"
        condition: and
