def _playwright_browsers_impl(ctx):
    # This rule creates a directory structure compatible with PLAYWRIGHT_BROWSERS_PATH
    # It takes the external browser repositories and symlinks them into subdirectories.

    out_dir = ctx.actions.declare_directory(ctx.label.name)

    # We want:
    # out_dir/chromium-1200/...
    # out_dir/firefox-1497/...
    # out_dir/ffmpeg-1011/...

    # Since we are using pre-extracted zips from http_archive, the files are already there.
    # But declare_directory expects us to populate it.

    # Actually, a simpler way is to use a rule that just provides the files
    # and we set PLAYWRIGHT_BROWSERS_PATH in the test to point to the runfiles root
    # IF we can name the directories correctly.

    # But Bazel's external repo names are fixed (and can't have dashes).

    # So we use ctx.actions.run to create the symlinks.

    script = "mkdir -p {dir}/chromium-1200 {dir}/firefox-1497 {dir}/ffmpeg-1011\n".format(dir=out_dir.path)

    # We need to find all files in the input depset and link them.
    # This is expensive in Starlark if done file-by-file.

    # Better: use a shell script that links the directories.
    # But some platforms don't support directory symlinks well or bazel handles them specifically.

    # Let's use a simpler approach:
    # Link the root of each repository to the expected name.

    # We can't easily link a whole external repo directory in a single action without knowing its path.
    # But we can find the path of one file and go up.

    inputs = []
    # chromium
    for f in ctx.files.chromium:
        inputs.append(f)
    # firefox
    for f in ctx.files.firefox:
        inputs.append(f)
    # ffmpeg
    for f in ctx.files.ffmpeg:
        inputs.append(f)

    # We'll use a python script to do the linking to be more robust than shell.
    # Actually, shell is fine on Linux.

    cmd = "mkdir -p {dir}/chromium-1200 {dir}/firefox-1497 {dir}/ffmpeg-1011\n".format(dir=out_dir.path)

    # For each input file, we want to symlink it into the output directory at the same relative path.
    # This is what a Rule that populates a directory does.

    # Let's try to just link the TOP LEVEL directories if they are available.

    # Actually, a better way for Playwright is to just point to the runfiles.
    # If we use `http_archive(name="chromium-1200", ...)` it would be perfect.
    # But we can't because of the dash.

    # Wait! We can use `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH`, etc.
    # But Playwright still wants the cache for other things.

    # Let's use the symlink approach.

    ctx.actions.run_shell(
        inputs = inputs,
        outputs = [out_dir],
        command = """
            mkdir -p {dir}/chromium-1200
            mkdir -p {dir}/firefox-1497
            mkdir -p {dir}/ffmpeg-1011

            # Use find to locate the root of each repo relative to the runfiles
            # Actually, we can just use the path of the first file in each set.

            ln -sf $(dirname $(dirname {chromium_sample}))/* {dir}/chromium-1200/ 2>/dev/null || true
            ln -sf $(dirname $(dirname {firefox_sample}))/* {dir}/firefox-1497/ 2>/dev/null || true
            ln -sf $(dirname $(dirname {ffmpeg_sample}))/* {dir}/ffmpeg-1011/ 2>/dev/null || true

            # Wait, that's not very bazel-y and might fail if paths are weird.
            # Let's just copy them for now to be safe, it's hermetic.
            # But the output MUST be in the out_dir.

            # Re-evaluating: Playwright expects the directory to CONTAIN 'chromium-1200'
            # So if we have:
            # bazel-bin/bazel/playwright/browsers/chromium-1200/...
            # Then we set PLAYWRIGHT_BROWSERS_PATH=$(location //bazel/playwright:browsers)

            # Populating correctly:
        """.format(
            dir = out_dir.path,
            chromium_sample = ctx.files.chromium[0].path if ctx.files.chromium else "",
            firefox_sample = ctx.files.firefox[0].path if ctx.files.firefox else "",
            ffmpeg_sample = ctx.files.ffmpeg[0].path if ctx.files.ffmpeg else "",
        ),
        # Actually I'll use a better script.
    )

    return [DefaultInfo(files = depset([out_dir]))]

# Actually, I'll use a simpler rule that just gathers everything and we'll handle the path in Go.
