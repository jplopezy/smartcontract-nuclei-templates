id: cosmos-unchecked-external-calls

info:
  name: Cosmos Unchecked External Calls Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Medium
  description: A contract calls an external address without validating the response, exposing it to attacks.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "require"
          - "invoke"
          - "runtime_upgrade"
        condition: and
