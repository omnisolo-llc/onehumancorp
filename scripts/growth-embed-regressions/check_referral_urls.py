"""Check rendered referral attributes only; never invoke capture or copy actions."""
from html.parser import HTMLParser
import json
import sys
from urllib.parse import parse_qsl, urlsplit

class Widget(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.links = []
        self.value = None
        self.tenant = None
    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag == 'a':
            self.links.append(attrs.get('href', ''))
        if tag == 'input' and attrs.get('id') == 'ref-link':
            assert 'readonly' in attrs
            self.value = attrs.get('value')
        if attrs.get('class') == 'widget-container':
            self.tenant = attrs.get('data-tenant')

sample = json.load(sys.stdin)
widget = Widget()
widget.feed(sample['html'])
assert len(widget.links) == 1
if sample['kind'] == 'post-purchase':
    assert widget.value is not None
    widget.links.append(widget.value)
else:
    assert sample['kind'] == 'birthday-club'
    assert widget.tenant == sample['tenant']
for link in widget.links:
    url = urlsplit(link)
    assert (url.scheme, url.netloc, url.path, url.fragment) == ('https', 'omnisolo.co', '/api/v1/growth/referrals/click', '')
    params = parse_qsl(url.query, keep_blank_values=True)
    assert params[:2] == [('target', '/onboarding'), ('ref', sample['tenant'])], params
    assert params[2:] == ([] if sample['kind'] == 'post-purchase' else [('source', 'birthday_club')]), params
print('Raw tenant round-trips through rendered referral URL attributes')
