id: cosmos-weak-authentication

info:
  name: Cosmos Weak Authentication Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract relies on insecure authentication mechanisms.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "log"
          - "storage"
          - "emit"
        condition: and
