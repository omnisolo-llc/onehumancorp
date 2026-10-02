"""Parse actual rendered HTML and test its static listener without external I/O."""
from html.parser import HTMLParser
import json
import subprocess
import sys
from urllib.parse import parse_qsl, urlsplit


class Widget(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.buttons = []
        self.anchors = []
        self.inline_handlers = []
        self.scripts = []
        self.in_script = False

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        self.inline_handlers.extend(key for key in attrs if key.lower().startswith("on"))
        if tag == "button":
            self.buttons.append(attrs)
        if tag == "a":
            self.anchors.append(attrs)
        if tag == "script":
            assert not attrs, "the widget needs only its local static listener"
            self.scripts.append("")
            self.in_script = True

    def handle_endtag(self, tag):
        if tag == "script":
            self.in_script = False

    def handle_data(self, text):
        if self.in_script:
            self.scripts[-1] += text


def main():
    sample = json.load(sys.stdin)
    widget = Widget()
    widget.feed(sample["html"])
    assert not widget.inline_handlers, f"dynamic inline event context remains: {widget.inline_handlers}"
    assert len(widget.buttons) == 1
    button = widget.buttons[0]
    assert button.get("id") == "referral-share"
    url = button["data-referral-url"]
    target = urlsplit(url)
    assert (target.scheme, target.netloc, target.path, target.fragment) == (
        "https", "omnisolo.co", "/api/v1/growth/referrals/click", ""
    )
    assert parse_qsl(target.query, keep_blank_values=True) == [("target", "/onboarding"), ("ref", sample["tenant"])]
    for anchor in widget.anchors:
        link = urlsplit(anchor.get("href", ""))
        assert (link.scheme, link.netloc, link.path, link.fragment) == (target.scheme, target.netloc, target.path, "")
        values = parse_qsl(link.query, keep_blank_values=True)
        assert values[:2] == [("target", "/onboarding"), ("ref", sample["tenant"])]
        assert values[2:] in ([], [("source", "viral_widget")])
    assert len(widget.scripts) == 1
    expected = """document.getElementById('referral-share').addEventListener('click', function() {
            window.open(this.dataset.referralUrl, '_blank');
        });"""
    assert " ".join(widget.scripts[0].split()) == " ".join(expected.split()), "listener must be static for every tenant"
    # No browser, fetch, provider, process or credential is exposed to this VM.
    # The recorder proves the real listener still opens exactly the parsed URL.
    runner = r"""
const vm = require('node:vm');
const fs = require('node:fs');
const {script, url} = JSON.parse(fs.readFileSync(0, 'utf8'));
let click;
const opened = [];
const button = {dataset: {referralUrl: url}, addEventListener(name, fn) {
  if (name !== 'click' || typeof fn !== 'function') throw new Error('Unexpected listener');
  click = fn;
}};
const context = vm.createContext({document: {getElementById(id) {
  if (id !== 'referral-share') throw new Error('Unexpected target');
  return button;
}}, window: {open(...args) {opened.push(args);}}});
new vm.Script(script).runInContext(context, {timeout: 1000});
if (opened.length !== 0 || !click) throw new Error('Action ran before a click');
click.call(button);
if (JSON.stringify(opened) !== JSON.stringify([[url, '_blank']])) throw new Error('Wrong destination/action');
if (context.injected !== undefined) throw new Error('Tenant data executed');
"""
    subprocess.run(["node", "-e", runner], input=json.dumps({"script": widget.scripts[0], "url": url}), text=True, check=True, timeout=10)
    print("HTML contexts and static click destination verified")


if __name__ == "__main__":
    main()
