"""Exercise the actual rendered birthday listener against loopback HTTP only."""
from html.parser import HTMLParser
import json
from pathlib import Path
import subprocess
import sys

class Widget(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.ids = {}
        self.stack = []
        self.scripts = []
        self.in_script = False
        self.tenant = None
    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        element_id = attrs.get('id')
        if element_id:
            self.ids[element_id] = {'text': '', 'style': attrs.get('style', '')}
        if attrs.get('class') == 'widget-container':
            self.tenant = attrs.get('data-tenant')
        if tag not in ('input', 'meta', 'link', 'br', 'img'):
            self.stack.append((tag, element_id))
        if tag == 'script':
            self.in_script = True
            self.scripts.append('')
    def handle_endtag(self, tag):
        if tag == 'script':
            self.in_script = False
        for index in range(len(self.stack) - 1, -1, -1):
            if self.stack[index][0] == tag:
                del self.stack[index:]
                break
    def handle_data(self, data):
        if self.in_script:
            self.scripts[-1] += data
        for _, element_id in self.stack:
            if element_id:
                self.ids[element_id]['text'] += data

sample = json.load(sys.stdin)
assert 'Sign up to receive a special gift' not in sample['html']
assert 'Configured birthday offer: 25% off.' in sample['html']
widget = Widget()
widget.feed(sample['html'])
assert len(widget.scripts) == 1
assert widget.tenant == sample['tenant']
payload = {'script': widget.scripts[0], 'ids': widget.ids, 'tenant': widget.tenant}
subprocess.run(['node', str(Path(__file__).with_name('birthday_capture_runner.cjs'))], input=json.dumps(payload), text=True, check=True, timeout=30)
