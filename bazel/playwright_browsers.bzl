load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def playwright_browsers():
    http_archive(
        name = "playwright_chromium",
        url = "https://playwright.azureedge.net/builds/chromium/1200/chromium-linux.zip",
        sha256 = "ab56b2a7955c2961f74348e2349f1e907489283da23dba37c730b624e4d670bb",
        build_file_content = "filegroup(name = 'files', srcs = glob(['**']), visibility = ['//visibility:public'])",
    )

    http_archive(
        name = "playwright_firefox",
        url = "https://playwright.azureedge.net/builds/firefox/1497/firefox-ubuntu-24.04.zip",
        sha256 = "168e69855d6e49997823fe765d66f02720f6add035a9d3ac94e51073d391a272",
        build_file_content = "filegroup(name = 'files', srcs = glob(['**']), visibility = ['//visibility:public'])",
    )

    http_archive(
        name = "playwright_ffmpeg",
        url = "https://playwright.azureedge.net/builds/ffmpeg/1011/ffmpeg-linux.zip",
        sha256 = "ebc74fc5b94830176a3c2914ae96bd8bc7f6a91f4f33890230f84a172ee61ccc",
        build_file_content = "filegroup(name = 'files', srcs = glob(['**']), visibility = ['//visibility:public'])",
    )

def _playwright_browsers_extension_impl(_module_ctx):
    playwright_browsers()

playwright_browsers_extension = module_extension(
    implementation = _playwright_browsers_extension_impl,
)
