id: ethereum-delegatecall-misuse

info:
  name: Ethereum Delegatecall Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract uses delegatecall in an unsafe manner, exposing itself to external execution risks.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "msg.sender"
          - "invoke"
          - "emit"
        condition: and
