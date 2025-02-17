id: polkadot-insufficient-validations

info:
  name: Polkadot Insufficient Validations Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract does not validate inputs properly, leading to potential exploits.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "require"
          - "runtime_upgrade"
          - "log"
        condition: and
