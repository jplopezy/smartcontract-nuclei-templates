id: ethereum-reentrancy

info:
  name: Ethereum Reentrancy Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: Low
  description: A contract allows external calls before updating state, enabling reentrancy attacks.

file:
  - extensions:
      - sol

    matchers:
      - type: word
        words:
          - "contract"
          - "key"
          - "owner"
        condition: and
