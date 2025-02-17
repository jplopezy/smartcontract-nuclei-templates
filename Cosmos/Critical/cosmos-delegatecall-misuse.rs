id: cosmos-delegatecall-misuse

info:
  name: Cosmos Delegatecall Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract uses delegatecall in an unsafe manner, exposing itself to external execution risks.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "key"
          - "trait"
          - "storage"
        condition: and
