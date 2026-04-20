package sandbox

import (
	"reflect"
	"testing"
)

func TestLinuxBwrapAdapter_BuildBwrapArgs(t *testing.T) {
	tests := []struct {
		name     string
		config   Config
		baseArgs []string
		want     []string
	}{
		{
			name:     "no seccomp config",
			config:   Config{EnableSeccomp: false, SeccompBPFPath: ""},
			baseArgs: []string{"--unshare-all"},
			want:     []string{"--unshare-all"},
		},
		{
			name:     "seccomp config enabled but no path",
			config:   Config{EnableSeccomp: true, SeccompBPFPath: ""},
			baseArgs: []string{"--unshare-all"},
			want:     []string{"--unshare-all"},
		},
		{
			name:     "seccomp config enabled with path",
			config:   Config{EnableSeccomp: true, SeccompBPFPath: "/path/to/filter.bpf"},
			baseArgs: []string{"--unshare-all"},
			want:     []string{"--unshare-all", "--seccomp", "/path/to/filter.bpf"},
		},
		{
			name:     "seccomp config disabled with path",
			config:   Config{EnableSeccomp: false, SeccompBPFPath: "/path/to/filter.bpf"},
			baseArgs: []string{"--unshare-all"},
			want:     []string{"--unshare-all"},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			a := &LinuxBwrapAdapter{
				Config: tt.config,
			}
			if got := a.BuildBwrapArgs(tt.baseArgs); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("LinuxBwrapAdapter.BuildBwrapArgs() = %v, want %v", got, tt.want)
			}
		})
	}
}
