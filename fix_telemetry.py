import sys

def process_file(filepath):
    with open(filepath, 'r') as f:
        lines = f.readlines()

    # Find the lines to move
    start_idx = -1
    for i, line in enumerate(lines):
        if "RagRecordsSyncedTotal, err = m.Int64Counter" in line:
            start_idx = i
            break

    end_idx = -1
    for i, line in enumerate(lines):
        if "var errs []error" in line:
            end_idx = i
            break

    if start_idx != -1 and end_idx != -1:
        block = lines[start_idx:end_idx]
        del lines[start_idx:end_idx]

        # the new errs_idx is where end_idx was, but shifted by the deleted block
        new_errs_idx = start_idx

        # We want to insert the block after the line `var errs []error`, which is at new_errs_idx
        for i, line in enumerate(block):
            lines.insert(new_errs_idx + 1 + i, line)

    with open(filepath, 'w') as f:
        f.writelines(lines)

process_file('srcs/server/telemetry/telemetry.go')
