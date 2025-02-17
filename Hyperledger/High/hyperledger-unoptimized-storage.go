id: hyperledger-unoptimized-storage

info:
  name: Hyperledger Unoptimized Storage Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract inefficiently manages storage, leading to higher gas costs.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "function"
          - "dispatch"
          - "msg.sender"
        condition: and
