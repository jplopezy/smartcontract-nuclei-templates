id: cosmos-unoptimized-storage

info:
  name: Cosmos Unoptimized Storage Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract inefficiently manages storage, leading to higher gas costs.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "mapping"
          - "function"
          - "trait"
        condition: and
