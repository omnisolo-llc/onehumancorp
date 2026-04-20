package sandbox

type Config struct {
	EnableSeccomp  bool
	SeccompBPFPath string
}

type LinuxBwrapAdapter struct {
	Config Config
}

func (a *LinuxBwrapAdapter) BuildBwrapArgs(baseArgs []string) []string {
	args := append([]string(nil), baseArgs...)
	if a.Config.EnableSeccomp && a.Config.SeccompBPFPath != "" {
		args = append(args, "--seccomp", a.Config.SeccompBPFPath)
	}
	return args
}
