id: cosmos-uninitialized-storage

info:
  name: Cosmos Uninitialized Storage Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract has uninitialized storage variables, which can be hijacked by attackers.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "require"
          - "msg.sender"
          - "signer"
        condition: and
