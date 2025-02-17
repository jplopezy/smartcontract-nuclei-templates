id: polkadot-code-style-issues

info:
  name: Polkadot Code Style Issues Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract has inconsistent formatting or coding style, reducing readability.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "storage"
          - "signer"
          - "mapping"
        condition: and
