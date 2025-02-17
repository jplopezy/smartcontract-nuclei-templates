id: solana-unchecked-external-calls

info:
  name: Solana Unchecked External Calls Vulnerability
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
          - "transaction"
          - "contract"
        condition: and
