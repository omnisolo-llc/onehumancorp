"""Reject clock dependency identities absent from the repository lockfile."""
from pathlib import Path
import tomllib

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PACKAGE = 'ohc-clock-receipt-regressions'


def verify(repository, focused):
    def identity(package):
        return (package['name'], package['version'], package.get('source'), package.get('checksum'))

    allowed = {identity(package) for package in repository['package']}
    extra = [identity(package) for package in focused['package']
             if package['name'] != PACKAGE and identity(package) not in allowed]
    if extra:
        raise ValueError(f'Clock dependency drift from root Cargo.lock: {extra}')
    roots = [package for package in focused['package'] if package['name'] == PACKAGE]
    if len(roots) != 1 or roots[0].get('source'):
        raise ValueError('Clock lock must contain exactly one local harness package')


if __name__ == '__main__':
    verify(tomllib.loads((ROOT / 'Cargo.lock').read_text()),
           tomllib.loads((HERE / 'Cargo.lock').read_text()))
    print('Clock dependency identities match repository Cargo.lock')
