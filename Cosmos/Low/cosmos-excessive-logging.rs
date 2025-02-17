id: cosmos-excessive-logging

info:
  name: Cosmos Excessive Logging Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract logs excessive data, increasing costs unnecessarily.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "runtime_upgrade"
          - "signer"
          - "invoke"
        condition: and
