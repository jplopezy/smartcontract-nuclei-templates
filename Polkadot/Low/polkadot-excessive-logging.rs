id: polkadot-excessive-logging

info:
  name: Polkadot Excessive Logging Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract logs excessive data, increasing costs unnecessarily.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "invoke"
          - "signer"
          - "send"
        condition: and
