id: polkadot-timestamp-manipulation

info:
  name: Polkadot Timestamp Manipulation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract relies on block.timestamp for critical operations, which miners can influence.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "owner"
          - "runtime_upgrade"
          - "mapping"
        condition: and
