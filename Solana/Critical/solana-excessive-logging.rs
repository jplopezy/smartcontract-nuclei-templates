id: solana-excessive-logging

info:
  name: Solana Excessive Logging Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract logs excessive data, increasing costs unnecessarily.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "signer"
          - "send"
          - "mapping"
        condition: and
