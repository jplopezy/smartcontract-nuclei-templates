id: polkadot-excessive-logging

info:
  name: Polkadot Excessive Logging Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract logs excessive data, increasing costs unnecessarily.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "send"
          - "mapping"
          - "log"
        condition: and
