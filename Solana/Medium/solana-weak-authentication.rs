id: solana-weak-authentication

info:
  name: Solana Weak Authentication Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract relies on insecure authentication mechanisms.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "owner"
          - "require"
          - "signer"
        condition: and
