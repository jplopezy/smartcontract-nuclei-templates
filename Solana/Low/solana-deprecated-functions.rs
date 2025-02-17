id: solana-deprecated-functions

info:
  name: Solana Deprecated Functions Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract uses deprecated Solidity functions that may be insecure.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "msg.sender"
          - "transaction"
          - "signer"
        condition: and
