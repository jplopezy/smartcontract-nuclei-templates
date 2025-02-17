id: hyperledger-unoptimized-storage

info:
  name: Hyperledger Unoptimized Storage Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract inefficiently manages storage, leading to higher gas costs.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "trait"
          - "contract"
          - "invoke"
        condition: and
