id: ethereum-unchecked-external-calls

info:
  name: Ethereum Unchecked External Calls Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract calls an external address without validating the response, exposing it to attacks.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "dispatch"
          - "emit"
          - "send"
        condition: and
