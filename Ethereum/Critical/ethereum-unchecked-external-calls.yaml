id: ethereum-unchecked-external-calls

info:
  name: Ethereum Unchecked External Calls Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract calls an external address without validating the response, exposing it to attacks.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "log"
          - "signer"
          - "storage"
        condition: and
