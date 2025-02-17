id: solana-event-emission-errors

info:
  name: Solana Event Emission Errors Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract fails to emit proper events, reducing auditability and transparency.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "msg.sender"
          - "send"
          - "invoke"
        condition: and
