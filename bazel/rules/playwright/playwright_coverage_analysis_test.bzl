"""Analysis test for Playwright coverage support-source wiring."""

load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")

_EXPECTED_SUPPORT_SUFFIX = [
    "$(rootpath //:playwright.config.ts)",
    "$(rootpath //src/e2e:authenticate.ts)",
    "$(rootpath //src/e2e:fixtures.ts)",
    "$(rootpath //src/e2e:global-setup.ts)",
]

_CoverageArgsInfo = provider(fields = ["args"])

def _coverage_args_aspect_impl(_target, ctx):
    return [_CoverageArgsInfo(args = ctx.rule.attr.args)]

_coverage_args_aspect = aspect(implementation = _coverage_args_aspect_impl)

def _support_sources_are_runtime_arguments_impl(ctx):
    env = analysistest.begin(ctx)
    argv = analysistest.target_under_test(env)[_CoverageArgsInfo].args

    support_positions = [index for index, argument in enumerate(argv) if argument == "--support"]
    asserts.equals(env, 1, len(support_positions), "coverage action must receive exactly one --support boundary")
    if support_positions:
        support_arguments = argv[support_positions[0] + 1:]
        for suffix in _EXPECTED_SUPPORT_SUFFIX:
            asserts.true(
                env,
                suffix in support_arguments,
                "coverage action is missing support source {} after --support".format(suffix),
            )
    return analysistest.end(env)

playwright_coverage_analysis_test = analysistest.make(
    _support_sources_are_runtime_arguments_impl,
    extra_target_under_test_aspects = [_coverage_args_aspect],
)
