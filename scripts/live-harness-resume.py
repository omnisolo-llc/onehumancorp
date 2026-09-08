#!/usr/bin/env python3
"""Validate a completed native gate before resuming only failed compatibility rows."""
import hashlib
import json
import os
from pathlib import Path
import sys

HARNESSES = ('omnisolo', 'codex', 'opencode', 'deepseek', 'pi', 'kimi', 'openhands', 'openharness', 'aider', 'goose', 'open-interpreter', 'plandex')
COMMON = {'mcp.catalog', 'mcp.invoke', 'integration.read', 'integration.invoke', 'browser.navigate', 'browser.snapshot'}
WRITE = COMMON | {'memory.write', 'artifact.write', 'workspace.write', 'cache.write'}
READ = COMMON | {'memory.search', 'artifact.read', 'workspace.read', 'cache.read'}


def token_usage(value, inside_tokens=False):
    if isinstance(value, dict):
        return any(token_usage(item, inside_tokens or 'token' in key.lower())
                   for key, item in value.items())
    if isinstance(value, list):
        return any(token_usage(item, inside_tokens) for item in value)
    return inside_tokens and isinstance(value, (int, float)) and not isinstance(value, bool) and value > 0


def validate(report, model, effort):
    def require(condition):
        if not condition:
            raise ValueError('resume report does not contain a complete verified native gate')

    require(report.get('schema') == 'omnisolo.live_harness_matrix.v1')
    require(report.get('status') == 'failed' and report.get('native_gate') == 'passed')
    require(report.get('model') == model and report.get('reasoning_effort') == effort)
    rows = report.get('results', [])
    require([row.get('harness_id') for row in rows] == list(HARNESSES))
    require(any(row.get('status') == 'failed' for row in rows[8:]))
    for index, row in enumerate(rows):
        require(row.get('model') == model and row.get('reasoning_effort') == effort)
        require(row.get('status') in ('passed', 'failed'))
        if index < 8:
            require(row.get('status') == 'passed' and row.get('integration_mode') == 'native')
        if row['status'] != 'passed':
            continue
        evidence = row.get('evidence', {})
        require(row.get('native_session_deleted') is True)
        require(evidence.get('credential_leak_observed') is False)
        require(all(evidence.get(field) is True for field in ('usage_observed', 'terminal_success_observed', 'provider_marker_observed')))
        require(token_usage(evidence.get('usage')))
        if index < 8:
            services = evidence.get('shared_local_services', {})
            require(services.get('status') == 'operations_verified')
            require(services.get('writer_verified') is True and services.get('reader_verified') is True)
            require(bool(services.get('writer_key')) and bool(services.get('writer_value')))
            if index:
                require(services.get('cross_harness_read_verified') is True)
            writer, reader = services.get('writer_operations', {}), services.get('reader_operations', {})
            require(bool(writer.get('attempt_id')) and bool(reader.get('attempt_id')))
            require(writer['attempt_id'] != reader['attempt_id'])
            require(WRITE <= set(writer.get('operations', [])))
            require(READ <= set(reader.get('operations', [])))
    return report


def prepare(path, model, effort):
    raw = Path(path).read_bytes()
    for name in ('OPENAI_API_KEY', 'SUB2API_API_KEY', 'OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN'):
        secret = os.environ.get(name)
        if secret and secret.encode() in raw:
            raise ValueError('resume report contains a credential')
    report = validate(json.loads(raw), model, effort)
    report['resumed_from'] = {'sha256': hashlib.sha256(raw).hexdigest(), 'native_gate': 'passed',
        'retained_harnesses': [row['harness_id'] for row in report['results'] if row['status'] == 'passed']}
    return report


if __name__ == '__main__':
    try:
        print(json.dumps(prepare(*sys.argv[1:])))
    except (OSError, ValueError, TypeError, KeyError, AttributeError):
        print('resume report rejected: complete native service receipts and matching model selection are required', file=sys.stderr)
        sys.exit(1)
