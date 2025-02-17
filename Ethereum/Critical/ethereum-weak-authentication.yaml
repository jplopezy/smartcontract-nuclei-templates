id: ethereum-weak-authentication

info:
  name: Ethereum Weak Authentication Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract relies on insecure authentication mechanisms.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "key"
          - "balance"
          - "send"
        condition: and
