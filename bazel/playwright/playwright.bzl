def _playwright_browsers_impl(ctx):
    out_dir = ctx.actions.declare_directory(ctx.label.name)
    
    inputs = []
    inputs.extend(ctx.files.chromium)
    inputs.extend(ctx.files.firefox)
    inputs.extend(ctx.files.ffmpeg)
    
    args = ctx.actions.args()
    args.add(out_dir.path)
    args.add_all(inputs)
    
    ctx.actions.run_shell(
        inputs = inputs,
        outputs = [out_dir],
        arguments = [args],
        command = """
        python3 -c "
import os, sys, shutil

dest_root = sys.argv[1]
srcs = sys.argv[2:]

repos = {}
for f in srcs:
    parts = f.split(os.sep)
    if 'external' in parts:
        idx = parts.index('external')
        if idx + 1 < len(parts):
            repo_name = parts[idx+1]
            if repo_name not in repos: repos[repo_name] = []
            repos[repo_name].append(f)

for repo_name, files in repos.items():
    common_dir = os.path.commonpath(files)
    if os.path.isfile(common_dir): common_dir = os.path.dirname(common_dir)
        
    target_sub = ''
    if 'playwright_chromium' in repo_name: target_sub = 'chromium-1200'
    elif 'playwright_firefox' in repo_name: target_sub = 'firefox-1497'
    elif 'playwright_ffmpeg' in repo_name: target_sub = 'ffmpeg-1011'
    
    if target_sub:
        dest_dir = os.path.join(dest_root, target_sub)
        os.makedirs(dest_dir, exist_ok=True)
        for item in os.listdir(common_dir):
            s = os.path.join(common_dir, item)
            d = os.path.join(dest_dir, item)
            if os.path.isdir(s): shutil.copytree(s, d, dirs_exist_ok=True)
            else: shutil.copy2(s, d)
" "$@"
        """,
    )
    
    return [DefaultInfo(files = depset([out_dir]))]

playwright_browsers = rule(
    implementation = _playwright_browsers_impl,
    attrs = {
        "chromium": attr.label(mandatory = True),
        "firefox": attr.label(mandatory = True),
        "ffmpeg": attr.label(mandatory = True),
    },
)
