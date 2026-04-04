import glob
import re

for filename in glob.glob('srcs/server/orchestration/*_test.go'):
    with open(filename, 'r') as f:
        content = f.read()

    # Replace db.NewTestProvider with NewTestProvider since we moved it here.
    content = content.replace('db.NewTestProvider', 'NewTestProvider')

    # Remove unused time import from autodream_kairos_test.go
    if filename.endswith('autodream_kairos_test.go'):
        content = content.replace('"time"\n', '')

    with open(filename, 'w') as f:
        f.write(content)
