id: polkadot-access-control-bypass

info:
  name: Polkadot Access Control Bypass Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: An access control mechanism is flawed, allowing unauthorized actions.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "storage"
          - "unchecked"
          - "emit"
        condition: and
