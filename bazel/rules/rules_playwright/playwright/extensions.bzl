"""Extensions for bzlmod.

Installs a playwright toolchain.
Every module can define a toolchain version under the default name, "playwright".
The latest of those versions will be selected (the rest discarded),
and will always be registered by rules_playwright.

Additionally, the root module can define arbitrarily many more toolchain versions under different
names (the latest version will be picked for each name) and can register them as it sees fit,
effectively overriding the default named toolchain due to toolchain resolution precedence.
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_file")
load("//playwright/private:known_browsers.bzl", "KNOWN_BROWSER_INTEGRITY")
load("//playwright/private:util.bzl", "get_all_cli_paths", "get_browsers_json_path", "get_cli_path")
load(":repositories.bzl", "playwright_repository")

_DEFAULT_NAME = "playwright"
_DEFAULT_BROWSER_URLS = [
    "https://cdn.playwright.dev",
    "https://playwright.azureedge.net",
    "https://playwright-akamai.azureedge.net",
    "https://playwright-verizon.azureedge.net",
]

def _browser_version(browsers_json, browser_name):
    for browser in browsers_json.get("browsers", []):
        if browser.get("name") == browser_name:
            return browser.get("browserVersion")
    return None

def _chrome_for_testing_path(path, browsers_json):
    if not path.startswith("builds/chromium/"):
        return path

    chromium_version = _browser_version(browsers_json, "chromium")
    headless_shell_version = _browser_version(browsers_json, "chromium-headless-shell") or chromium_version

    if not chromium_version:
        return path

    if path.endswith("/chromium-linux.zip"):
        return "builds/cft/{}/linux64/chrome-linux64.zip".format(chromium_version)
    if path.endswith("/chromium-headless-shell-linux.zip") and headless_shell_version:
        return "builds/cft/{}/linux64/chrome-headless-shell-linux64.zip".format(headless_shell_version)

    return path

playwright_repo = tag_class(attrs = {
    "name": attr.string(doc = """\
Base name for generated repositories, allowing more than one playwright toolchain to be registered.
Overriding the default is only permitted in the root module.
""", default = _DEFAULT_NAME),
    "playwright_version": attr.string(doc = "Explicit version of playwright to download browsers.json from"),
    "browsers_download_urls": attr.string_list(
        default = _DEFAULT_BROWSER_URLS,
        doc = "URLs to download playwright browsers from. Replace defaults if a mirror location is preferred.",
    ),
    "browsers_json": attr.label(doc = "Alternative to playwright_version. Skips downloading from unpkg", allow_single_file = True),
    "integrity_map": attr.string_dict(
        default = {},
        doc = "Deprecated: Mapping from brower target to integrity hash",
    ),
    "integrity_path_map": attr.string_dict(
        default = {},
        doc = "Mapping from browser path to integrity hash",
    ),
})

def _extension_impl(module_ctx):
    for mod in module_ctx.modules:
        for repo in mod.tags.repo:
            name = repo.name

            if name != _DEFAULT_NAME and not mod.is_root:
                fail("""\
                Only the root module may override the default name for the playwright toolchain.
                This prevents conflicting registrations in the global namespace of external repos.
                """)

            if not repo.playwright_version and not repo.browsers_json:
                fail("""\
                One of playwright_version or browsers_json must be specified.
                """)

            cli = get_cli_path(module_ctx)

            # Watch all CLI binaries to ensure MODULE.bazel.lock remains consistent
            # across platforms and detects changes when binaries are updated
            for cli_path in get_all_cli_paths(module_ctx):
                module_ctx.watch(cli_path)

            browsers_json_path = get_browsers_json_path(module_ctx, repo.playwright_version, repo.browsers_json)
            browsers_json = json.decode(module_ctx.read(browsers_json_path))

            # Step 1: use module_ctx exec to get the list of browsers to iterate over and declare with http file
            result = module_ctx.execute(
                [
                    cli,
                    "http-files",
                    "--browser-json-path",
                    browsers_json_path,
                    "--browsers-workspace-name-prefix",
                    repo.name,
                ],
            )
            if result.return_code != 0:
                fail("http-files command failed", result.stdout, result.stderr)

            for http_file_json in json.decode(result.stdout):
                browser_name = http_file_json["name"]
                path = _chrome_for_testing_path(http_file_json["path"], browsers_json)
                integrity = repo.integrity_map.get(browser_name, None)
                if not integrity:
                    integrity = repo.integrity_path_map.get(path, None)
                    if not integrity:
                        integrity = KNOWN_BROWSER_INTEGRITY.get(path, None)

                urls = [url + "/" + path for url in repo.browsers_download_urls]

                http_file(
                    name = browser_name,
                    integrity = integrity,
                    urls = urls,
                )

            # Step 2: generate repository which references said http_files
            playwright_repository(
                name = name,
                playwright_version = repo.playwright_version,
                browsers_json = repo.browsers_json,
                browsers_workspace_name_prefix = name,
                # Map the apparant name of the module to the cannonical name
                # See https://bazel.build/external/module
                rules_playwright_cannonical_name = "@" + Label("rules_playwright").repo_name,
            )

playwright = module_extension(
    implementation = _extension_impl,
    tag_classes = {"repo": playwright_repo},
)
