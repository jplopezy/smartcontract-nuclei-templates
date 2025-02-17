id: solana-access-control-bypass

info:
  name: Solana Access Control Bypass Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: An access control mechanism is flawed, allowing unauthorized actions.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "balance"
          - "unchecked"
          - "require"
        condition: and
