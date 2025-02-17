id: polkadot-unchecked-external-calls

info:
  name: Polkadot Unchecked External Calls Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract calls an external address without validating the response, exposing it to attacks.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "send"
          - "msg.sender"
          - "transaction"
        condition: and
