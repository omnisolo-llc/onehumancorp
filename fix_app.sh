# Now let's test the flutter app code but only models to avoid full GUI test overhead for memory.
~/go/bin/bazelisk test --jobs=4 //srcs/app/lib/models:agent_model_test
