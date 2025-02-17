id: polkadot-deprecated-functions

info:
  name: Polkadot Deprecated Functions Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract uses deprecated Solidity functions that may be insecure.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "trait"
          - "signer"
          - "dispatch"
        condition: and
