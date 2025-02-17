id: ethereum-deprecated-functions

info:
  name: Ethereum Deprecated Functions Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract uses deprecated Solidity functions that may be insecure.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "runtime_upgrade"
          - "send"
          - "transaction"
        condition: and
