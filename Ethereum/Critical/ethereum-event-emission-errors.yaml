id: ethereum-event-emission-errors

info:
  name: Ethereum Event Emission Errors Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Critical
  description: A contract fails to emit proper events, reducing auditability and transparency.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "key"
          - "runtime_upgrade"
          - "unchecked"
        condition: and
