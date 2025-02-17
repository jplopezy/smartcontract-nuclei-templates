id: ethereum-timestamp-manipulation

info:
  name: Ethereum Timestamp Manipulation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract relies on block.timestamp for critical operations, which miners can influence.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "msg.sender"
          - "unchecked"
          - "trait"
        condition: and
