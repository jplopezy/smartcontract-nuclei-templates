import re
import unittest
from pathlib import Path
import yaml

ROOT = Path(__file__).resolve().parents[1]


class SignatureTests(unittest.TestCase):
    def load(self, relative):
        data = yaml.safe_load((ROOT / relative).read_text(encoding='utf-8'))
        self.assertEqual(data['info']['severity'], 'info')
        self.assertIn('source-review', data['info']['tags'])
        self.assertTrue(all(ext.startswith('.') for ext in data['file'][0]['extensions']))
        regexes = data['file'][0]['matchers'][0]['regex']
        for pattern in regexes:
            re.compile(pattern)
        return regexes

    def match(self, regexes, text):
        return any(re.search(r, text) for r in regexes)

    def test_tx_origin(self):
        rx = self.load('Ethereum/Review/ethereum-tx-origin-auth.yaml')
        self.assertTrue(self.match(rx, 'require(tx.origin == owner);'))
        self.assertFalse(self.match(rx, 'require(msg.sender == owner);'))

    def test_call_order(self):
        rx = self.load('Ethereum/Review/ethereum-ether-call-then-state-write.yaml')
        self.assertTrue(self.match(rx, 'recipient.call{value: amount}(""); balances[msg.sender] -= amount;'))
        self.assertFalse(self.match(rx, 'balances[msg.sender] -= amount; recipient.call{value: amount}("");'))

    def test_unchecked_calls(self):
        rx = self.load('Ethereum/Review/ethereum-unchecked-call-review.yaml')
        self.assertTrue(self.match(rx, 'recipient.call{value: amount}("");'))
        self.assertFalse(self.match(rx, '(bool ok,) = recipient.call{value: amount}("");'))

    def test_solana_unchecked(self):
        rx = self.load('Solana/Review/solana-unchecked-account-data-review.yaml')
        self.assertTrue(self.match(rx, 'let x = Account::try_deserialize_unchecked(&mut data)?;'))
        self.assertFalse(self.match(rx, 'let x = Account::try_deserialize(&mut data)?;'))


if __name__ == '__main__':
    unittest.main()
