#!/bin/bash
npx @bazel/bazelisk test //src/e2e:playwright --local_test_jobs="$(nproc)" --test_output=errors
