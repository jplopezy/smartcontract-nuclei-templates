id: hyperledger-weak-authentication

info:
  name: Hyperledger Weak Authentication Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract relies on insecure authentication mechanisms.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "runtime_upgrade"
          - "require"
          - "mapping"
        condition: and
