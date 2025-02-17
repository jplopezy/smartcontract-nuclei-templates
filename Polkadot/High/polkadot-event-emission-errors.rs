id: polkadot-event-emission-errors

info:
  name: Polkadot Event Emission Errors Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract fails to emit proper events, reducing auditability and transparency.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "storage"
          - "function"
          - "invoke"
        condition: and
