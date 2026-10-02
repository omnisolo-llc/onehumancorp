#!/usr/bin/env python3
"""Run one source-bound offline/isolated-DB gate and reject missing test coverage."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

# Minimum existing executed inventory. Raising coverage must never hide removal
# of an existing test; these are floors, not filters passed to the Rust harness.
GATES = {
    'builder-generation-contract': (50, 'OHC_BUILDER_GENERATION_TEST_DATABASE_URL'),
    'widget-chat-contract': (32, 'OHC_WIDGET_TEST_DATABASE_URL'),
    'order-milestones': (13, 'OHC_MILESTONE_TEST_DATABASE_URL'),
    'operations-appointments': (11, 'OHC_APPOINTMENTS_TEST_DATABASE_URL'),
    'agent-definition-contract': (49, 'OHC_AGENT_DEFINITION_TEST_DATABASE_URL'),

    'chat-tenant-isolation': (13, 'OHC_CHAT_TEST_DATABASE_URL'),
    'bootstrap-portable-roles': (9, 'OHC_SETUP_TEST_DATABASE_URL'),
    'link-bio-isolation': (14, 'OHC_BIO_TEST_DATABASE_URL'),
    'tenant-search': (13, 'OHC_SEARCH_TEST_DATABASE_URL'),
    'onboarding-durability': (70, 'OHC_SYNC_TEST_DATABASE_URL'),
    'quote-acceptance': (25, 'OHC_QUOTE_TEST_DATABASE_URL'),
    'onboarding-draft-merge': (6, 'OHC_DRAFT_TEST_DATABASE_URL'),
    'agent-workflow-contract': (97, None),
    'agent-receipt-postgres-contract': (50, 'OHC_AGENT_RECEIPT_TEST_DATABASE_URL'),
    'stripe-webhook-security': (22, None),
    'builder-publication': (5, 'OHC_BUILDER_TEST_DATABASE_URL'),
    'site-publication': (81, 'OHC_PUBLICATION_TEST_DATABASE_URL'),
    'service-creation': (6, 'OHC_SERVICE_TEST_DATABASE_URL'),
}
RESULT = re.compile(r'test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;')


def validate_results(log, minimum):
    results = RESULT.findall(log)
    if not results:
        raise ValueError('No Rust test execution summary; discovery/execution is unverified')
    passed = 0
    for status, count, failed, ignored, measured, filtered in results:
        if status != 'ok' or any(int(n) for n in (failed, ignored, measured, filtered)):
            raise ValueError('Failed, ignored, measured-only or filtered tests cannot certify this gate')
        passed += int(count)
    if passed < minimum:
        raise ValueError(f'Executed {passed} tests; required inventory is at least {minimum}')
    return passed


def run_gate(root, name, evidence):
    root = Path(root).resolve()
    destination = Path(evidence).resolve() / name
    destination.mkdir(parents=True, exist_ok=True)
    minimum, _ = GATES[name]
    receipt = {'gate': name, 'status': 'failed', 'minimum_tests': minimum}
    try:
        process = subprocess.Popen(['bash', str(root/'scripts'/name/'run.sh')], cwd=root,
                                   stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        chunks = []
        with (destination/'run.log').open('w') as output, process.stdout as stream:
            for line in stream:
                output.write(line)
                output.flush()
                sys.stdout.write(line)
                sys.stdout.flush()
                chunks.append(line)
        receipt['exit_code'] = process.wait()
        if receipt['exit_code'] != 0:
            raise ValueError(f"Runner exited {receipt['exit_code']}")
        receipt['passed'] = validate_results(''.join(chunks), minimum)
        manifest = (root/'scripts'/name/'source-manifest.json').read_bytes()
        if not json.loads(manifest):
            raise ValueError('Source manifest is empty')
        (destination/'source-manifest.json').write_bytes(manifest)
        receipt['source_manifest_sha256'] = hashlib.sha256(manifest).hexdigest()
        receipt['status'] = 'passed'
    except (OSError, ValueError) as error:
        receipt['error'] = str(error)
        print(f'{name}: {error}', file=sys.stderr)
    finally:
        (destination/'result.json').write_text(json.dumps(receipt, indent=2)+'\n')
    return 0 if receipt['status'] == 'passed' else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('gate', choices=GATES)
    parser.add_argument('--evidence', type=Path, default=Path('target/focused-ci-results'))
    args = parser.parse_args()
    return run_gate(Path(__file__).resolve().parents[1], args.gate, args.evidence)


if __name__ == '__main__':
    sys.exit(main())
