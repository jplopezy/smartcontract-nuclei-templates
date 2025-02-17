id: hyperledger-reentrancy

info:
  name: Hyperledger Reentrancy Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract allows external calls before updating state, enabling reentrancy attacks.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "invoke"
          - "dispatch"
          - "balance"
        condition: and
