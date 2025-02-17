id: solana-delegatecall-misuse

info:
  name: Solana Delegatecall Misuse Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract uses delegatecall in an unsafe manner, exposing itself to external execution risks.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "msg.sender"
          - "send"
          - "contract"
        condition: and
