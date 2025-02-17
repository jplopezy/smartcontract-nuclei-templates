id: hyperledger-unchecked-external-calls

info:
  name: Hyperledger Unchecked External Calls Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract calls an external address without validating the response, exposing it to attacks.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "require"
          - "key"
          - "transaction"
        condition: and
