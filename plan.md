Oh! It was `ql-owo-lp` (another user or automated script, possibly `jules-auto-rebase` bot) that committed those `.bazelrc` and `.github` changes!
Wait, `0b74c1bc9d0d91a17df8ceb8240d4b8340d126e0` contains all those `.github` modifications!

To fix this, I need to fetch the latest `origin/main` (which I already did), checkout `jules-6998033235004007715-4877fb03`, hard reset to `origin/main`, and then re-apply ONLY MY changes!
Wait!
"Please revert these changes to proceed." means I should remove those files from the PR branch.
If I just `git checkout origin/main .github .bazelrc .gitignore buildbuddy.yaml` and commit, I will revert them!
Let's do that!
