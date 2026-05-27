ANDROID_API_LEVEL = 35
ANDROID_BUILD_TOOLS_VERSION = "35.0.0"

# Tauri's generated Android/AGP project still asks for Build Tools 35 in some
# dependency modules, so provision it in the Bazel SDK to keep Gradle offline.
ANDROID_COMPAT_BUILD_TOOLS_VERSIONS = [
    "35.0.0",
]

ANDROID_NDK_VERSION = "27.0.12077973"
ANDROID_RUST_TARGET = "aarch64-linux-android"

ANDROID_CMDLINE_TOOLS_URL = "https://dl.google.com/android/repository/commandlinetools-linux-14742923_latest.zip"
ANDROID_CMDLINE_TOOLS_SHA256 = "04453066b540409d975c676d781da1477479dde3761310f1a7eb92a1dfb15af7"

ANDROID_GRADLE_VERSION = "8.14.3"
ANDROID_GRADLE_DISTRIBUTION_SHA256 = "bd71102213493060956ec229d946beee57158dbd89d0e62b91bca0fa2c5f3531"
ANDROID_GRADLE_DISTRIBUTION_URLS = [
    "https://services.gradle.org/distributions/gradle-%s-bin.zip" % ANDROID_GRADLE_VERSION,
]

ANDROID_JDK_SHA256 = "67e810b31427ac0ff1c249473595066a00bdf0f9265df186c32905d5f75c93b8"
ANDROID_JDK_STRIP_PREFIX = "zulu21.46.19-ca-jdk21.0.9-linux_x64"
ANDROID_JDK_URLS = [
    "https://cdn.azul.com/zulu/bin/zulu21.46.19-ca-jdk21.0.9-linux_x64.tar.gz",
    "https://mirror.bazel.build/cdn.azul.com/zulu/bin/zulu21.46.19-ca-jdk21.0.9-linux_x64.tar.gz",
]
