id: cosmos-event-emission-errors

info:
  name: Cosmos Event Emission Errors Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract fails to emit proper events, reducing auditability and transparency.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "dispatch"
          - "send"
          - "contract"
        condition: and
