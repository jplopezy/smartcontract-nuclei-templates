id: cosmos-code-style-issues

info:
  name: Cosmos Code Style Issues Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract has inconsistent formatting or coding style, reducing readability.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "unchecked"
          - "key"
          - "log"
        condition: and
