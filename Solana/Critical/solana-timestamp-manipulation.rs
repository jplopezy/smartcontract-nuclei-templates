id: solana-timestamp-manipulation

info:
  name: Solana Timestamp Manipulation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract relies on block.timestamp for critical operations, which miners can influence.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "owner"
          - "emit"
          - "storage"
        condition: and
