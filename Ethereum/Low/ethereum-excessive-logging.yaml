id: ethereum-excessive-logging

info:
  name: Ethereum Excessive Logging Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract logs excessive data, increasing costs unnecessarily.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "function"
          - "owner"
          - "invoke"
        condition: and
