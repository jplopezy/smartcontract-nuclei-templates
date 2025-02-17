id: hyperledger-delegatecall-misuse

info:
  name: Hyperledger Delegatecall Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract uses delegatecall in an unsafe manner, exposing itself to external execution risks.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "dispatch"
          - "log"
          - "contract"
        condition: and
