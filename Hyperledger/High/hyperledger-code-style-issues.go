id: hyperledger-code-style-issues

info:
  name: Hyperledger Code Style Issues Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract has inconsistent formatting or coding style, reducing readability.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "log"
          - "owner"
          - "trait"
        condition: and
