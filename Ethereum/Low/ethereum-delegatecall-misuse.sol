id: ethereum-delegatecall-misuse

info:
  name: Ethereum Delegatecall Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract uses delegatecall in an unsafe manner, exposing itself to external execution risks.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "require"
          - "signer"
          - "contract"
        condition: and
