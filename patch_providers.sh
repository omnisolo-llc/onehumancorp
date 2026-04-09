sed -i 's/if !strings.HasPrefix(absClean, absBase) {/if !strings.HasPrefix(absClean, absBase+string(filepath.Separator)) \&\& absClean != absBase {/g' srcs/server/tools/hybridfsmcp/local_provider.go
sed -i 's/if !strings.HasPrefix(absClean, absBase) {/if !strings.HasPrefix(absClean, absBase+string(filepath.Separator)) \&\& absClean != absBase {/g' srcs/server/tools/hybridfsmcp/cloud_provider.go
