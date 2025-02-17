id: solana-event-emission-errors

info:
  name: Solana Event Emission Errors Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract fails to emit proper events, reducing auditability and transparency.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "key"
          - "transaction"
          - "trait"
        condition: and
