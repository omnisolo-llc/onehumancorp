package main

import (
	"context"
	"fmt"
	"os"
	"sync"
)

var (
	standaloneThrottle     = make(chan struct{}, 1)
	standaloneThrottleOnce sync.Once
)

func acquireThrottle(ctx context.Context) error {
	standaloneThrottleOnce.Do(func() {
		if os.Getenv("OHC_STANDALONE") == "true" {
			// already initialized to 1
		}
	})

	if os.Getenv("OHC_STANDALONE") == "true" {
		select {
		case standaloneThrottle <- struct{}{}:
			fmt.Println("Acquired")
			return nil
		case <-ctx.Done():
			return ctx.Err()
		}
	}
	return nil
}

func releaseThrottle() {
	if os.Getenv("OHC_STANDALONE") == "true" {
		<-standaloneThrottle
		fmt.Println("Released")
	}
}

func main() {
	os.Setenv("OHC_STANDALONE", "true")
	ctx := context.Background()

	acquireThrottle(ctx)
	releaseThrottle()
	acquireThrottle(ctx)
	releaseThrottle()
}
