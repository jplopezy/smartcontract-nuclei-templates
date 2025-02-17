id: hyperledger-uninitialized-storage

info:
  name: Hyperledger Uninitialized Storage Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract has uninitialized storage variables, which can be hijacked by attackers.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "runtime_upgrade"
          - "owner"
          - "unchecked"
        condition: and
