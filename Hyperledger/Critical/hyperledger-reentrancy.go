id: hyperledger-reentrancy

info:
  name: Hyperledger Reentrancy Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract allows external calls before updating state, enabling reentrancy attacks.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "signer"
          - "invoke"
          - "trait"
        condition: and
