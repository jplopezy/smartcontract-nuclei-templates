id: cosmos-insufficient-validations

info:
  name: Cosmos Insufficient Validations Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract does not validate inputs properly, leading to potential exploits.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "dispatch"
          - "owner"
          - "runtime_upgrade"
        condition: and
