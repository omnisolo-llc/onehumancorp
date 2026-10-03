"""Parse real authenticated draft HTML without a browser or network."""
from html.parser import HTMLParser
import json
import sys

class Draft(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.text = []
        self.buttons = 0
        self.anchors = []

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        assert tag not in ('script', 'iframe', 'form', 'object'), tag
        assert not any(key.lower().startswith('on') for key in attrs), attrs
        if tag == 'button':
            assert 'disabled' in attrs
            assert 'data-referral-url' not in attrs
            self.buttons += 1
        if tag == 'input':
            assert attrs.get('value') == ''
            assert 'readonly' in attrs and 'disabled' in attrs
        if tag == 'a':
            self.anchors.append(attrs.get('href'))

    def handle_data(self, data):
        self.text.append(data)

sample = json.load(sys.stdin)
parsed = Draft()
parsed.feed(sample['html'])
assert parsed.buttons == 1
assert parsed.anchors == ['https://omnisolo.co']
assert sample['supplied'] in ''.join(parsed.text)
assert '/referrals/click' not in sample['html']
assert 'not configured' in ''.join(parsed.text)
print('Literal draft fields and absence of uncreated referral actions verified')
