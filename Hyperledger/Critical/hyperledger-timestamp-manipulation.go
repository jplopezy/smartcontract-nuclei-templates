id: hyperledger-timestamp-manipulation

info:
  name: Hyperledger Timestamp Manipulation Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract relies on block.timestamp for critical operations, which miners can influence.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "dispatch"
          - "runtime_upgrade"
          - "send"
        condition: and
