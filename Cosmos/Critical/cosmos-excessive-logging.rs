id: cosmos-excessive-logging

info:
  name: Cosmos Excessive Logging Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract logs excessive data, increasing costs unnecessarily.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "emit"
          - "contract"
          - "runtime_upgrade"
        condition: and
