id: polkadot-weak-authentication

info:
  name: Polkadot Weak Authentication Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract relies on insecure authentication mechanisms.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "runtime_upgrade"
          - "transaction"
          - "signer"
        condition: and
