id: polkadot-unoptimized-storage

info:
  name: Polkadot Unoptimized Storage Vulnerability
  author: Juan Pablo Lopez Yacubian
  severity: High
  description: A contract inefficiently manages storage, leading to higher gas costs.

file:
  - extensions:
      - rs

    matchers:
      - type: word
        words:
          - "contract"
          - "runtime_upgrade"
          - "invoke"
        condition: and
