id: hyperledger-insufficient-validations

info:
  name: Hyperledger Insufficient Validations Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract does not validate inputs properly, leading to potential exploits.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "msg.sender"
          - "signer"
          - "invoke"
        condition: and
