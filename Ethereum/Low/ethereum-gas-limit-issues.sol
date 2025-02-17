id: ethereum-gas-limit-issues

info:
  name: Ethereum Gas Limit Issues Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract has loops or operations that can exceed the gas limit, leading to failed transactions.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "log"
          - "invoke"
          - "mapping"
        condition: and
