import sys

def main():
    with open('BUILD.bazel', 'r') as f:
        lines = f.readlines()

    out = []
    skip = False
    for line in lines:
        if line.startswith('go_test('):
            if 'name = "mono_test"' in ''.join(lines):
                # We need to find if this specific go_test is mono_test
                pass # it's hard to parse this way, let's use sed instead to just remove the last 4 lines

    with open('BUILD.bazel', 'w') as f:
        f.writelines(out)

if __name__ == "__main__":
    pass
