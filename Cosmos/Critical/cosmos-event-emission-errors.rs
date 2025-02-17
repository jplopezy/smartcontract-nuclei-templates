id: cosmos-event-emission-errors

info:
  name: Cosmos Event Emission Errors Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract fails to emit proper events, reducing auditability and transparency.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "transaction"
          - "dispatch"
          - "balance"
        condition: and
