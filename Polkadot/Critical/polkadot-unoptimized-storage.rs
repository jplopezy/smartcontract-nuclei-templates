id: polkadot-unoptimized-storage

info:
  name: Polkadot Unoptimized Storage Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract inefficiently manages storage, leading to higher gas costs.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "storage"
          - "trait"
          - "unchecked"
        condition: and
