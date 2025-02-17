id: polkadot-delegatecall-misuse

info:
  name: Polkadot Delegatecall Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract uses delegatecall in an unsafe manner, exposing itself to external execution risks.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "log"
          - "balance"
          - "trait"
        condition: and
