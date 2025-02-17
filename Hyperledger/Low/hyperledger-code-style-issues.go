id: hyperledger-code-style-issues

info:
  name: Hyperledger Code Style Issues Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract has inconsistent formatting or coding style, reducing readability.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "send"
          - "contract"
          - "runtime_upgrade"
        condition: and
