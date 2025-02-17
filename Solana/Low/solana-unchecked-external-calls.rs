id: solana-unchecked-external-calls

info:
  name: Solana Unchecked External Calls Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract calls an external address without validating the response, exposing it to attacks.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "msg.sender"
          - "function"
          - "log"
        condition: and
