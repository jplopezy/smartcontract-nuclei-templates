id: cosmos-weak-authentication

info:
  name: Cosmos Weak Authentication Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract relies on insecure authentication mechanisms.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "trait"
          - "log"
          - "send"
        condition: and
