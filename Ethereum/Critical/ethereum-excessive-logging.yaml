id: ethereum-excessive-logging

info:
  name: Ethereum Excessive Logging Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract logs excessive data, increasing costs unnecessarily.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "runtime_upgrade"
          - "invoke"
          - "trait"
        condition: and
