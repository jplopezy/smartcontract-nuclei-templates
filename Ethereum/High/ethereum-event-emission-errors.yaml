id: ethereum-event-emission-errors

info:
  name: Ethereum Event Emission Errors Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract fails to emit proper events, reducing auditability and transparency.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "emit"
          - "transaction"
          - "msg.sender"
        condition: and
