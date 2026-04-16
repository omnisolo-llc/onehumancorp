package profiling

import (
	"log"
	"os"
	"runtime/pprof"
)

// StartCPUProfile starts writing a CPU profile to the given file.
func StartCPUProfile(file string) func() {
	f, err := os.Create(file)
	if err != nil {
		log.Fatalf("could not create CPU profile: %v", err)
	}
	if err := pprof.StartCPUProfile(f); err != nil {
		log.Fatalf("could not start CPU profile: %v", err)
	}
	return func() {
		pprof.StopCPUProfile()
		f.Close()
	}
}

// WriteMemProfile writes a memory profile to the given file.
func WriteMemProfile(file string) {
	f, err := os.Create(file)
	if err != nil {
		log.Fatalf("could not create memory profile: %v", err)
	}
	defer f.Close()
	if err := pprof.WriteHeapProfile(f); err != nil {
		log.Fatalf("could not write memory profile: %v", err)
	}
}
