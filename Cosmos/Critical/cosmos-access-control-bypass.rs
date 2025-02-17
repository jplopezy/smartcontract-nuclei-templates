id: cosmos-access-control-bypass

info:
  name: Cosmos Access Control Bypass Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: An access control mechanism is flawed, allowing unauthorized actions.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "storage"
          - "unchecked"
          - "runtime_upgrade"
        condition: and
