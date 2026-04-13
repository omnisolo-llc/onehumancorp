package agentruntime

import (
	"context"
	"encoding/json"
	"errors"
	"io"
	"net/http"
	"strings"
	"testing"
)

func TestDefaultRegionAutoPrefersAvailableOCIRuntime(t *testing.T) {
	launcher := NewLauncher(Options{
		LookPath: func(name string) (string, error) {
			if name == "podman" {
				return "/usr/bin/podman", nil
			}
			return "", errors.New("missing")
		},
	})

	if got := launcher.DefaultRegion(); got != "podman" {
		t.Fatalf("expected podman region, got %q", got)
	}
}

func TestDefaultRegionAutoFallsBackToSandboxThenProcess(t *testing.T) {
	sandbox := NewLauncher(Options{
		LookPath: func(name string) (string, error) {
			if name == "bwrap" {
				return "/usr/bin/bwrap", nil
			}
			return "", errors.New("missing")
		},
	})
	if got := sandbox.DefaultRegion(); got != "sandbox" {
		t.Fatalf("expected sandbox region, got %q", got)
	}

	process := NewLauncher(Options{
		LookPath: func(string) (string, error) { return "", errors.New("missing") },
	})
	if got := process.DefaultRegion(); got != "process" {
		t.Fatalf("expected process region, got %q", got)
	}
}

func TestLaunchTaskBuildsContainerCommand(t *testing.T) {
	var captured execSpec
	launcher := NewLauncher(Options{
		Runtime: "podman",
		Environ: []string{
			"OPENAI_API_KEY=test-key",
			"UNRELATED=value",
		},
		Exec: func(_ context.Context, spec execSpec) error {
			captured = spec
			return nil
		},
	})

	err := launcher.LaunchTask(context.Background(), TaskRequest{
		AgentID:     "agent-1",
		Description: "Fix issue",
		Prompt:      "Do the work",
		WorkDir:     "/repo",
	})
	if err != nil {
		t.Fatalf("LaunchTask returned error: %v", err)
	}

	if captured.Path != "podman" {
		t.Fatalf("expected podman executable, got %q", captured.Path)
	}
	joined := strings.Join(captured.Args, " ")
	for _, want := range []string{"run", "--rm", "-v /repo:/repo", "-w /repo", "onehumancorp/internal-default-agent:bazel", "/usr/local/bin/ohc-agent-task"} {
		if !strings.Contains(joined, want) {
			t.Fatalf("expected %q in command %q", want, joined)
		}
	}
	if !strings.Contains(joined, "--task-json-base64=") {
		t.Fatalf("expected task payload argument in %q", joined)
	}
	if !strings.Contains(joined, "OPENAI_API_KEY=test-key") {
		t.Fatalf("expected OPENAI_API_KEY to be forwarded in %q", joined)
	}
	if strings.Contains(joined, "UNRELATED=value") {
		t.Fatalf("unexpected unrelated env in %q", joined)
	}
}

func TestLaunchTaskBuildsKubernetesJobManifest(t *testing.T) {
	var captured execSpec
	launcher := NewLauncher(Options{
		Runtime:        "kubernetes",
		Namespace:      "agents",
		WorkspaceClaim: "workspace-pvc",
		K8sWorkDir:     "/workspace",
		Exec: func(_ context.Context, spec execSpec) error {
			captured = spec
			return nil
		},
	})

	err := launcher.LaunchTask(context.Background(), TaskRequest{
		AgentID:     "agent-1",
		IssueID:     "issue-9",
		Description: "Investigate cluster issue",
		Prompt:      "Fix it",
		WorkDir:     "/repo",
	})
	if err != nil {
		t.Fatalf("LaunchTask returned error: %v", err)
	}

	if captured.Path != "kubectl" {
		t.Fatalf("expected kubectl executable, got %q", captured.Path)
	}
	if len(captured.Stdin) == 0 {
		t.Fatal("expected kubernetes manifest on stdin")
	}

	var manifest map[string]interface{}
	if err := json.Unmarshal(captured.Stdin, &manifest); err != nil {
		t.Fatalf("failed to parse manifest: %v", err)
	}
	metadata := manifest["metadata"].(map[string]interface{})
	if metadata["namespace"] != "agents" {
		t.Fatalf("expected namespace agents, got %v", metadata["namespace"])
	}
	spec := manifest["spec"].(map[string]interface{})
	template := spec["template"].(map[string]interface{})
	podSpec := template["spec"].(map[string]interface{})
	containers := podSpec["containers"].([]interface{})
	container := containers[0].(map[string]interface{})
	if container["workingDir"] != "/workspace" {
		t.Fatalf("expected workingDir /workspace, got %v", container["workingDir"])
	}
	volumes := podSpec["volumes"].([]interface{})
	volume := volumes[0].(map[string]interface{})
	claim := volume["persistentVolumeClaim"].(map[string]interface{})
	if claim["claimName"] != "workspace-pvc" {
		t.Fatalf("expected workspace PVC, got %v", claim["claimName"])
	}
}

func TestLaunchTaskUsesInClusterKubernetesAPIWhenKubectlUnavailable(t *testing.T) {
	var capturedRequest *http.Request
	var capturedBody []byte

	launcher := NewLauncher(Options{
		Runtime:   "kubernetes",
		Namespace: "agents",
		InCluster: true,
		K8sHost:   "kubernetes.default.svc",
		K8sPort:   "443",
		K8sToken:  "test-token",
		LookPath: func(string) (string, error) {
			return "", errors.New("missing")
		},
		HTTPDo: func(req *http.Request) (*http.Response, error) {
			capturedRequest = req
			var err error
			capturedBody, err = io.ReadAll(req.Body)
			if err != nil {
				return nil, err
			}
			return &http.Response{
				StatusCode: http.StatusCreated,
				Body:       io.NopCloser(strings.NewReader(`{"status":"ok"}`)),
			}, nil
		},
	})

	err := launcher.LaunchTask(context.Background(), TaskRequest{
		AgentID:     "agent-1",
		IssueID:     "issue-7",
		Description: "Investigate in-cluster launch",
		Prompt:      "Run in cluster",
		WorkDir:     "/repo",
	})
	if err != nil {
		t.Fatalf("LaunchTask returned error: %v", err)
	}
	if capturedRequest == nil {
		t.Fatal("expected kubernetes API request to be sent")
	}
	if capturedRequest.Method != http.MethodPost {
		t.Fatalf("expected POST request, got %s", capturedRequest.Method)
	}
	if got := capturedRequest.URL.String(); got != "https://kubernetes.default.svc:443/apis/batch/v1/namespaces/agents/jobs" {
		t.Fatalf("unexpected kubernetes job endpoint: %s", got)
	}
	if auth := capturedRequest.Header.Get("Authorization"); auth != "Bearer test-token" {
		t.Fatalf("unexpected authorization header: %q", auth)
	}

	var manifest map[string]interface{}
	if err := json.Unmarshal(capturedBody, &manifest); err != nil {
		t.Fatalf("failed to parse manifest body: %v", err)
	}
	if manifest["kind"] != "Job" {
		t.Fatalf("expected job manifest, got %v", manifest["kind"])
	}
}
