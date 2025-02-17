id: cosmos-gas-limit-issues

info:
  name: Cosmos Gas Limit Issues Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract has loops or operations that can exceed the gas limit, leading to failed transactions.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "runtime_upgrade"
          - "send"
          - "invoke"
        condition: and
