import yaml
with open('.github/workflows/ci.yml', 'r') as f:
    data = yaml.safe_load(f)
for job in data['jobs'].values():
    for step in job['steps']:
        if 'run' in step and 'bazelisk test' in step['run']:
            step['run'] = step['run'].replace('bazelisk test //...', 'bazelisk test --jobs=200 //...')
with open('.github/workflows/ci.yml', 'w') as f:
    yaml.dump(data, f)
