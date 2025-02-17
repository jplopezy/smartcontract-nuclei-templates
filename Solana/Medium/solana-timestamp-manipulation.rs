id: solana-timestamp-manipulation

info:
  name: Solana Timestamp Manipulation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract relies on block.timestamp for critical operations, which miners can influence.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "key"
          - "log"
          - "invoke"
        condition: and
