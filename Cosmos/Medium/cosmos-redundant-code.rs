id: cosmos-redundant-code

info:
  name: Cosmos Redundant Code Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract contains redundant logic, making maintenance difficult.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "storage"
          - "contract"
          - "dispatch"
        condition: and
