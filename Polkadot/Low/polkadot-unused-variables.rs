id: polkadot-unused-variables

info:
  name: Polkadot Unused Variables Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract has unused variables, increasing contract size and potential confusion.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "dispatch"
          - "invoke"
          - "trait"
        condition: and
