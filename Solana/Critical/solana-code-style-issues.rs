id: solana-code-style-issues

info:
  name: Solana Code Style Issues Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract has inconsistent formatting or coding style, reducing readability.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "log"
          - "owner"
          - "contract"
        condition: and
