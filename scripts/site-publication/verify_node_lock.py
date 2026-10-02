"""Bind the tiny CI witness to the maintained client's exact registry package."""
from pathlib import Path
import json


def verify(here, client):
    expected = '5.1.0'
    records = []
    for directory in [here, client]:
        manifest = json.loads((directory/'package.json').read_text())
        lock = json.loads((directory/'package-lock.json').read_text())
        assert manifest['dependencies']['canonicalize'] == expected, 'Unpinned publication serializer'
        assert lock['packages']['']['dependencies']['canonicalize'] == expected, 'Unpaired serializer lock'
        entry = lock['packages']['node_modules/canonicalize']
        assert entry['version'] == expected, 'Unexpected serializer version'
        assert entry['resolved'] == f'https://registry.npmjs.org/canonicalize/-/canonicalize-{expected}.tgz'
        assert entry['integrity'].startswith('sha512-'), 'Missing npm package integrity'
        records.append(entry)
    assert records[0] == records[1], 'Witness does not match maintained client package'


if __name__ == '__main__':
    here = Path(__file__).resolve().parent
    verify(here, here.parents[1]/'src/ui/next')
    print('Publication witness matches the maintained client canonicalize 5.1.0 lock')
