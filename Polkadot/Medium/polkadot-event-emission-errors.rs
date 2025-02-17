id: polkadot-event-emission-errors

info:
  name: Polkadot Event Emission Errors Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract fails to emit proper events, reducing auditability and transparency.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "function"
          - "emit"
          - "contract"
        condition: and
