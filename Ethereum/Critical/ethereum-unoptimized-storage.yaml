id: ethereum-unoptimized-storage

info:
  name: Ethereum Unoptimized Storage Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract inefficiently manages storage, leading to higher gas costs.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "send"
          - "storage"
          - "owner"
        condition: and
