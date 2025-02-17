id: cosmos-access-control-bypass

info:
  name: Cosmos Access Control Bypass Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: An access control mechanism is flawed, allowing unauthorized actions.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "owner"
          - "storage"
          - "invoke"
        condition: and
