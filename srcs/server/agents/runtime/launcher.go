package agentruntime

import (
	"bytes"
	"context"
	"crypto/tls"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"hash/fnv"
	"io"
	"net/http"
	"net/url"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
)

type Mode string

const (
	ModeKubernetes    Mode = "kubernetes"
	ModeKubernetesAPI Mode = "kubernetes-api"
	ModeOCI           Mode = "oci"
	ModeSandbox       Mode = "sandbox"
	ModeProcess       Mode = "process"
)

const (
	defaultImage         = "onehumancorp/internal-default-agent:bazel"
	defaultK8sNamespace  = "default"
	defaultK8sWorkDir    = "/workspace"
	defaultWorkerBinary  = "ohc-agent-task"
	defaultSandboxBinary = "bwrap"
	defaultKubectlBinary = "kubectl"
	defaultK8sHTTPSPort  = "443"
	defaultK8sCAPath     = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt"
	defaultK8sTokenPath  = "/var/run/secrets/kubernetes.io/serviceaccount/token"
)

type TaskRequest struct {
	AgentID      string `json:"agentId"`
	AgentName    string `json:"agentName,omitempty"`
	Role         string `json:"role,omitempty"`
	ProviderType string `json:"providerType,omitempty"`
	IssueID      string `json:"issueId,omitempty"`
	Description  string `json:"description"`
	Prompt       string `json:"prompt"`
	WorkDir      string `json:"workDir,omitempty"`
}

type Options struct {
	Runtime        string
	WorkerBinary   string
	Image          string
	Namespace      string
	ServiceAccount string
	WorkspaceClaim string
	K8sWorkDir     string
	LookPath       func(string) (string, error)
	Exec           func(context.Context, execSpec) error
	HTTPDo         func(*http.Request) (*http.Response, error)
	Environ        []string
	InCluster      bool
	K8sHost        string
	K8sPort        string
	K8sToken       string
	K8sCAPath      string
}

type Launcher struct {
	opts options
}

type options struct {
	runtime        string
	workerBinary   string
	image          string
	namespace      string
	serviceAccount string
	workspaceClaim string
	k8sWorkDir     string
	lookPath       func(string) (string, error)
	exec           func(context.Context, execSpec) error
	httpDo         func(*http.Request) (*http.Response, error)
	environ        []string
	inCluster      bool
	k8sHost        string
	k8sPort        string
	k8sToken       string
	k8sCAPath      string
}

type backend struct {
	mode   Mode
	path   string
	region string
}

type execSpec struct {
	Path  string
	Args  []string
	Env   []string
	Dir   string
	Stdin []byte
}

func NewLauncherFromEnv() *Launcher {
	return NewLauncher(Options{
		Runtime:        os.Getenv("OHC_AGENT_RUNTIME"),
		WorkerBinary:   os.Getenv("OHC_AGENT_TASK_BINARY"),
		Image:          os.Getenv("OHC_AGENT_IMAGE"),
		Namespace:      os.Getenv("OHC_AGENT_K8S_NAMESPACE"),
		ServiceAccount: os.Getenv("OHC_AGENT_K8S_SERVICE_ACCOUNT"),
		WorkspaceClaim: os.Getenv("OHC_AGENT_WORKSPACE_CLAIM"),
		K8sWorkDir:     os.Getenv("OHC_AGENT_K8S_WORKDIR"),
		InCluster:      os.Getenv("KUBERNETES_SERVICE_HOST") != "",
		K8sHost:        os.Getenv("KUBERNETES_SERVICE_HOST"),
		K8sPort:        firstNonEmpty(os.Getenv("KUBERNETES_SERVICE_PORT_HTTPS"), os.Getenv("KUBERNETES_SERVICE_PORT")),
		K8sCAPath:      defaultK8sCAPath,
	})
}

func NewLauncher(input Options) *Launcher {
	workerBinary := input.WorkerBinary
	if workerBinary == "" {
		workerBinary = defaultWorkerBinaryPath()
	}

	image := input.Image
	if image == "" {
		image = defaultImage
	}

	namespace := input.Namespace
	if namespace == "" {
		namespace = defaultK8sNamespace
	}

	k8sWorkDir := input.K8sWorkDir
	if k8sWorkDir == "" {
		k8sWorkDir = defaultK8sWorkDir
	}

	lookPath := input.LookPath
	if lookPath == nil {
		lookPath = exec.LookPath
	}

	execFn := input.Exec
	if execFn == nil {
		execFn = defaultExec
	}

	environ := input.Environ
	if environ == nil {
		environ = os.Environ()
	}

	k8sPort := input.K8sPort
	if k8sPort == "" {
		k8sPort = defaultK8sHTTPSPort
	}

	k8sCAPath := input.K8sCAPath
	if k8sCAPath == "" {
		k8sCAPath = defaultK8sCAPath
	}

	runtime := normalizeRuntime(input.Runtime)
	if runtime == "" {
		runtime = "auto"
	}

	return &Launcher{opts: options{
		runtime:        runtime,
		workerBinary:   workerBinary,
		image:          image,
		namespace:      namespace,
		serviceAccount: input.ServiceAccount,
		workspaceClaim: input.WorkspaceClaim,
		k8sWorkDir:     k8sWorkDir,
		lookPath:       lookPath,
		exec:           execFn,
		httpDo:         input.HTTPDo,
		environ:        environ,
		inCluster:      input.InCluster,
		k8sHost:        input.K8sHost,
		k8sPort:        k8sPort,
		k8sToken:       input.K8sToken,
		k8sCAPath:      k8sCAPath,
	}}
}

func (l *Launcher) DefaultRegion() string {
	return l.resolveBackend().region
}

func (l *Launcher) LaunchTask(ctx context.Context, req TaskRequest) error {
	selected := l.resolveBackend()
	payloadRequest := req
	if selected.mode == ModeKubernetes || selected.mode == ModeKubernetesAPI {
		payloadRequest.WorkDir = l.opts.k8sWorkDir
	}

	payload, err := encodeTaskRequest(payloadRequest)
	if err != nil {
		return err
	}

	if selected.mode == ModeKubernetesAPI {
		return l.launchKubernetesAPI(ctx, req, payload)
	}

	spec, err := l.buildExecSpec(selected, req, payload)
	if err != nil {
		return err
	}

	return l.opts.exec(ctx, spec)
}

func (l *Launcher) resolveBackend() backend {
	switch l.opts.runtime {
	case "kubernetes", "k8s":
		return l.kubernetesBackend()
	case "docker":
		return l.binaryBackend(ModeOCI, "docker", "docker")
	case "podman":
		return l.binaryBackend(ModeOCI, "podman", "podman")
	case "nerdctl":
		return l.binaryBackend(ModeOCI, "nerdctl", "nerdctl")
	case "oci", "container":
		if candidate, ok := l.firstAvailableOCI(); ok {
			return candidate
		}
		return l.binaryBackend(ModeOCI, "docker", "docker")
	case "sandbox", "bubblewrap", "bwrap":
		return l.binaryBackend(ModeSandbox, defaultSandboxBinary, "sandbox")
	case "process":
		return backend{mode: ModeProcess, path: l.opts.workerBinary, region: "process"}
	default:
		if l.opts.inCluster {
			return l.kubernetesBackend()
		}
		if candidate, ok := l.firstAvailableOCI(); ok {
			return candidate
		}
		if l.binaryExists(defaultSandboxBinary) {
			return l.binaryBackend(ModeSandbox, defaultSandboxBinary, "sandbox")
		}
		return backend{mode: ModeProcess, path: l.opts.workerBinary, region: "process"}
	}
}

func (l *Launcher) kubernetesBackend() backend {
	if l.binaryExists(defaultKubectlBinary) {
		return l.binaryBackend(ModeKubernetes, defaultKubectlBinary, "kubernetes")
	}
	if l.opts.inCluster && l.opts.k8sHost != "" {
		return backend{mode: ModeKubernetesAPI, region: "kubernetes"}
	}
	return l.binaryBackend(ModeKubernetes, defaultKubectlBinary, "kubernetes")
}

func (l *Launcher) firstAvailableOCI() (backend, bool) {
	for _, candidate := range []string{"docker", "podman", "nerdctl"} {
		if l.binaryExists(candidate) {
			return l.binaryBackend(ModeOCI, candidate, candidate), true
		}
	}
	return backend{}, false
}

func (l *Launcher) binaryBackend(mode Mode, binary, region string) backend {
	if path, err := l.opts.lookPath(binary); err == nil {
		return backend{mode: mode, path: path, region: region}
	}
	return backend{mode: mode, path: binary, region: region}
}

func (l *Launcher) binaryExists(binary string) bool {
	_, err := l.opts.lookPath(binary)
	return err == nil
}

func (l *Launcher) buildExecSpec(selected backend, req TaskRequest, payload string) (execSpec, error) {
	switch selected.mode {
	case ModeKubernetes:
		return l.buildKubernetesSpec(selected, req, payload)
	case ModeOCI:
		return l.buildContainerSpec(selected, req, payload)
	case ModeSandbox:
		return l.buildSandboxSpec(selected, req, payload)
	default:
		return l.buildProcessSpec(req, payload)
	}
}

func (l *Launcher) launchKubernetesAPI(ctx context.Context, req TaskRequest, payload string) error {
	manifest, err := l.jobManifest(req, payload)
	if err != nil {
		return err
	}

	endpoint, err := l.kubernetesJobsEndpoint()
	if err != nil {
		return err
	}

	token, err := l.kubernetesToken()
	if err != nil {
		return err
	}

	httpReq, err := http.NewRequestWithContext(ctx, http.MethodPost, endpoint, bytes.NewReader(manifest))
	if err != nil {
		return fmt.Errorf("create kubernetes request: %w", err)
	}
	httpReq.Header.Set("Authorization", "Bearer "+token)
	httpReq.Header.Set("Content-Type", "application/json")

	resp, err := l.doHTTPRequest(httpReq)
	if err != nil {
		return fmt.Errorf("launch agent task with kubernetes api: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= http.StatusOK && resp.StatusCode < http.StatusMultipleChoices {
		return nil
	}

	body, _ := io.ReadAll(io.LimitReader(resp.Body, 16<<10))
	trimmed := strings.TrimSpace(string(body))
	if trimmed == "" {
		trimmed = resp.Status
	}
	return fmt.Errorf("launch agent task with kubernetes api: %s: %s", resp.Status, trimmed)
}

func (l *Launcher) buildProcessSpec(req TaskRequest, payload string) (execSpec, error) {
	workDir, err := hostWorkDir(req.WorkDir)
	if err != nil {
		return execSpec{}, err
	}

	return execSpec{
		Path: l.opts.workerBinary,
		Args: []string{"--task-json-base64=" + payload},
		Env:  l.opts.environ,
		Dir:  workDir,
	}, nil
}

func (l *Launcher) buildSandboxSpec(selected backend, req TaskRequest, payload string) (execSpec, error) {
	workDir, err := hostWorkDir(req.WorkDir)
	if err != nil {
		return execSpec{}, err
	}

	args := []string{
		"--die-with-parent",
		"--new-session",
		"--proc", "/proc",
		"--dev", "/dev",
		"--tmpfs", "/tmp",
		"--ro-bind", "/", "/",
	}
	if workDir != "" {
		args = append(args,
			"--bind", workDir, workDir,
			"--chdir", workDir,
		)
	}
	args = append(args, l.opts.workerBinary, "--task-json-base64="+payload)

	return execSpec{
		Path: selected.path,
		Args: args,
		Env:  l.opts.environ,
	}, nil
}

func (l *Launcher) buildContainerSpec(selected backend, req TaskRequest, payload string) (execSpec, error) {
	workDir, err := hostWorkDir(req.WorkDir)
	if err != nil {
		return execSpec{}, err
	}

	args := []string{"run", "--rm", "--name", containerName(req)}
	for _, envVar := range forwardedEnv(l.opts.environ) {
		args = append(args, "-e", envVar)
	}
	if workDir != "" {
		args = append(args,
			"-v", workDir+":"+workDir,
			"-w", workDir,
		)
	}
	args = append(args,
		l.opts.image,
		"/usr/local/bin/ohc-agent-task",
		"--task-json-base64="+payload,
	)

	return execSpec{
		Path: selected.path,
		Args: args,
		Env:  l.opts.environ,
	}, nil
}

func (l *Launcher) buildKubernetesSpec(selected backend, req TaskRequest, payload string) (execSpec, error) {
	manifest, err := l.jobManifest(req, payload)
	if err != nil {
		return execSpec{}, err
	}

	return execSpec{
		Path:  selected.path,
		Args:  []string{"apply", "-f", "-"},
		Env:   l.opts.environ,
		Stdin: manifest,
	}, nil
}

func (l *Launcher) jobManifest(req TaskRequest, payload string) ([]byte, error) {
	container := map[string]interface{}{
		"name":    "agent-task",
		"image":   l.opts.image,
		"command": []string{"/usr/local/bin/ohc-agent-task"},
		"args":    []string{"--task-json-base64=" + payload},
	}
	if envVars := envMaps(forwardedEnv(l.opts.environ)); len(envVars) != 0 {
		container["env"] = envVars
	}

	podSpec := map[string]interface{}{
		"restartPolicy": "Never",
		"containers":    []map[string]interface{}{container},
	}
	if l.opts.serviceAccount != "" {
		podSpec["serviceAccountName"] = l.opts.serviceAccount
	}
	if l.opts.k8sWorkDir != "" {
		container["workingDir"] = l.opts.k8sWorkDir
		container["volumeMounts"] = []map[string]interface{}{map[string]interface{}{
			"name":      "workspace",
			"mountPath": l.opts.k8sWorkDir,
		}}
		volume := map[string]interface{}{"name": "workspace"}
		if l.opts.workspaceClaim != "" {
			volume["persistentVolumeClaim"] = map[string]interface{}{"claimName": l.opts.workspaceClaim}
		} else {
			volume["emptyDir"] = map[string]interface{}{}
		}
		podSpec["volumes"] = []map[string]interface{}{volume}
	}

	manifest := map[string]interface{}{
		"apiVersion": "batch/v1",
		"kind":       "Job",
		"metadata": map[string]interface{}{
			"name":      jobName(req),
			"namespace": l.opts.namespace,
			"labels": map[string]string{
				"app.kubernetes.io/name":       "ohc-agent-task",
				"app.kubernetes.io/managed-by": "ohc",
			},
		},
		"spec": map[string]interface{}{
			"backoffLimit":            0,
			"ttlSecondsAfterFinished": 300,
			"template": map[string]interface{}{
				"metadata": map[string]interface{}{
					"labels": map[string]string{
						"app.kubernetes.io/name": "ohc-agent-task",
					},
				},
				"spec": podSpec,
			},
		},
	}

	return json.Marshal(manifest)
}

func encodeTaskRequest(req TaskRequest) (string, error) {
	raw, err := json.Marshal(req)
	if err != nil {
		return "", fmt.Errorf("marshal task request: %w", err)
	}
	return base64.StdEncoding.EncodeToString(raw), nil
}

func (l *Launcher) kubernetesJobsEndpoint() (string, error) {
	if l.opts.k8sHost == "" {
		return "", fmt.Errorf("missing kubernetes service host")
	}
	return fmt.Sprintf("https://%s:%s/apis/batch/v1/namespaces/%s/jobs", l.opts.k8sHost, l.opts.k8sPort, url.PathEscape(l.opts.namespace)), nil
}

func (l *Launcher) kubernetesToken() (string, error) {
	if l.opts.k8sToken != "" {
		return l.opts.k8sToken, nil
	}
	raw, err := os.ReadFile(defaultK8sTokenPath)
	if err != nil {
		return "", fmt.Errorf("read kubernetes service account token: %w", err)
	}
	return strings.TrimSpace(string(raw)), nil
}

func (l *Launcher) doHTTPRequest(req *http.Request) (*http.Response, error) {
	if l.opts.httpDo != nil {
		return l.opts.httpDo(req)
	}

	client, err := l.kubernetesHTTPClient()
	if err != nil {
		return nil, err
	}
	return client.Do(req)
}

func (l *Launcher) kubernetesHTTPClient() (*http.Client, error) {
	caPEM, err := os.ReadFile(l.opts.k8sCAPath)
	if err != nil {
		return nil, fmt.Errorf("read kubernetes CA certificate: %w", err)
	}

	pool := x509.NewCertPool()
	if !pool.AppendCertsFromPEM(caPEM) {
		return nil, fmt.Errorf("parse kubernetes CA certificate")
	}

	transport, ok := http.DefaultTransport.(*http.Transport)
	if !ok {
		return nil, fmt.Errorf("default http transport does not support cloning")
	}

	clone := transport.Clone()
	clone.TLSClientConfig = &tls.Config{
		MinVersion: tls.VersionTLS12,
		RootCAs:    pool,
	}

	return &http.Client{
		Timeout:   30 * time.Second,
		Transport: clone,
	}, nil
}

func forwardedEnv(environ []string) []string {
	allowedPrefixes := []string{
		"ALL_PROXY=",
		"ANTHROPIC_API_BASE_URL=",
		"ANTHROPIC_API_KEY=",
		"GEMINI_API_KEY=",
		"HTTPS_PROXY=",
		"HTTP_PROXY=",
		"MINIMAX_API_KEY=",
		"NO_PROXY=",
		"OHC_LOCAL_",
		"OPENAI_API_BASE=",
		"OPENAI_API_KEY=",
	}

	var forwarded []string
	for _, envVar := range environ {
		for _, prefix := range allowedPrefixes {
			if strings.HasPrefix(envVar, prefix) {
				forwarded = append(forwarded, envVar)
				break
			}
		}
	}
	return forwarded
}

func envMaps(environ []string) []map[string]string {
	vars := make([]map[string]string, 0, len(environ))
	for _, envVar := range environ {
		parts := strings.SplitN(envVar, "=", 2)
		if len(parts) != 2 {
			continue
		}
		vars = append(vars, map[string]string{
			"name":  parts[0],
			"value": parts[1],
		})
	}
	return vars
}

func hostWorkDir(input string) (string, error) {
	if input == "" {
		return "", nil
	}
	if filepath.IsAbs(input) {
		return input, nil
	}
	resolved, err := filepath.Abs(input)
	if err != nil {
		return "", fmt.Errorf("resolve work dir %q: %w", input, err)
	}
	return resolved, nil
}

func containerName(req TaskRequest) string {
	return "ohc-agent-" + shortHash(req.AgentID+":"+req.IssueID+":"+req.Description)
}

func jobName(req TaskRequest) string {
	return "ohc-agent-task-" + shortHash(req.AgentID+":"+req.IssueID+":"+req.Description)
}

func shortHash(input string) string {
	hash := fnv.New32a()
	_, _ = hash.Write([]byte(input))
	return fmt.Sprintf("%08x", hash.Sum32())
}

func normalizeRuntime(runtime string) string {
	return strings.ToLower(strings.TrimSpace(runtime))
}

func firstNonEmpty(values ...string) string {
	for _, value := range values {
		if value != "" {
			return value
		}
	}
	return ""
}

func defaultWorkerBinaryPath() string {
	if exe, err := os.Executable(); err == nil {
		candidate := filepath.Join(filepath.Dir(exe), defaultWorkerBinary)
		if _, statErr := os.Stat(candidate); statErr == nil {
			return candidate
		}
	}
	return defaultWorkerBinary
}

func defaultExec(ctx context.Context, spec execSpec) error {
	cmd := exec.CommandContext(ctx, spec.Path, spec.Args...)
	if len(spec.Env) != 0 {
		cmd.Env = spec.Env
	}
	if spec.Dir != "" {
		cmd.Dir = spec.Dir
	}
	if len(spec.Stdin) != 0 {
		cmd.Stdin = bytes.NewReader(spec.Stdin)
	}
	output, err := cmd.CombinedOutput()
	if err != nil {
		trimmed := strings.TrimSpace(string(output))
		if trimmed == "" {
			return fmt.Errorf("launch agent task with %s: %w", spec.Path, err)
		}
		return fmt.Errorf("launch agent task with %s: %w: %s", spec.Path, err, trimmed)
	}
	return nil
}
