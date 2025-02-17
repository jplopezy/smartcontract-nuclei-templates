id: solana-unoptimized-storage

info:
  name: Solana Unoptimized Storage Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract inefficiently manages storage, leading to higher gas costs.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "emit"
          - "balance"
          - "storage"
        condition: and
