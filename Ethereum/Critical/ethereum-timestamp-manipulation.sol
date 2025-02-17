id: ethereum-timestamp-manipulation

info:
  name: Ethereum Timestamp Manipulation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract relies on block.timestamp for critical operations, which miners can influence.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "signer"
          - "balance"
          - "require"
        condition: and
