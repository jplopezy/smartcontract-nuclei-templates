id: polkadot-code-style-issues

info:
  name: Polkadot Code Style Issues Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract has inconsistent formatting or coding style, reducing readability.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "storage"
          - "balance"
          - "trait"
        condition: and
