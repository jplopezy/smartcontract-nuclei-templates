id: cosmos-timestamp-manipulation

info:
  name: Cosmos Timestamp Manipulation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract relies on block.timestamp for critical operations, which miners can influence.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "unchecked"
          - "key"
          - "balance"
        condition: and
