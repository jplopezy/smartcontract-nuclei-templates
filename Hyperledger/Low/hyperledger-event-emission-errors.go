id: hyperledger-event-emission-errors

info:
  name: Hyperledger Event Emission Errors Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract fails to emit proper events, reducing auditability and transparency.

file:
  - extensions:
      - go

    matchers:
      - type: word
        words:
          - "function"
          - "contract"
          - "unchecked"
        condition: and
