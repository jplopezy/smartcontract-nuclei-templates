id: cosmos-weak-authentication

info:
  name: Cosmos Weak Authentication Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract relies on insecure authentication mechanisms.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "trait"
          - "log"
          - "key"
        condition: and
