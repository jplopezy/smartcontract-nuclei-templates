id: cosmos-delegatecall-misuse

info:
  name: Cosmos Delegatecall Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract uses delegatecall in an unsafe manner, exposing itself to external execution risks.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "runtime_upgrade"
          - "signer"
          - "owner"
        condition: and
