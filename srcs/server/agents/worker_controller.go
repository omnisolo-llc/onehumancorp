package agents

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
	"log/slog"
	"net"
	"net/http"
	"net/url"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
	"github.com/onehumancorp/mono/srcs/server/agents/builtinclient"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

const (
	defaultWorkerBinaryName = "ohc-agent-task"
	defaultAgentImage       = "onehumancorp/internal-default-agent:bazel"
	defaultHubGRPCAddress   = "127.0.0.1:9090"
	defaultK8sNamespace     = "default"
	defaultK8sWorkDir       = "/workspace"
	defaultKubectlBinary    = "kubectl"
	defaultK8sHTTPSPort     = "443"
	defaultK8sCAPath        = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt"
	defaultK8sTokenPath     = "/var/run/secrets/kubernetes.io/serviceaccount/token"
)

type WorkerController interface {
	EnsureProvisioned(context.Context, orchestration.Agent) error
	Deprovision(context.Context, string) error
}

type workerController struct {
	hub *orchestration.Hub

	mu      sync.Mutex
	handles map[string]managedWorker
	opts    workerControllerOptions
}

type workerControllerOptions struct {
	runtime        string
	workerBinary   string
	image          string
	namespace      string
	serviceAccount string
	workspaceClaim string
	k8sWorkDir     string
	hubAddress     string
	builtinAddress string
	lookPath       func(string) (string, error)
	environ        []string
	inCluster      bool
	k8sHost        string
	k8sPort        string
	k8sCAPath      string
	httpDo         func(*http.Request) (*http.Response, error)
}

type managedWorker interface {
	Alive() bool
	Stop(context.Context) error
}

type inlineWorkerHandle struct {
	cancel context.CancelFunc
	done   chan struct{}

	mu    sync.Mutex
	alive bool
}

type processWorkerHandle struct {
	cmd  *exec.Cmd
	done chan struct{}

	mu    sync.Mutex
	alive bool
}

type kubernetesWorkerHandle struct {
	name string
	opts workerControllerOptions

	mu    sync.Mutex
	alive bool
}

type workerProcessConfig struct {
	AgentID        string `json:"agentId"`
	AgentName      string `json:"agentName,omitempty"`
	Role           string `json:"role,omitempty"`
	OrganizationID string `json:"organizationId,omitempty"`
	ProviderType   string `json:"providerType,omitempty"`
	Region         string `json:"region,omitempty"`
	HubAddress     string `json:"hubAddress"`
	BuiltinAddress string `json:"builtinAddress,omitempty"`
}

func NewWorkerControllerFromEnv(hub *orchestration.Hub) WorkerController {
	return &workerController{
		hub:     hub,
		handles: make(map[string]managedWorker),
		opts: workerControllerOptions{
			runtime:        defaultManagedRuntimeRegion(),
			workerBinary:   envOrDefault("OHC_AGENT_TASK_BINARY", defaultWorkerBinaryPath()),
			image:          envOrDefault("OHC_AGENT_IMAGE", defaultAgentImage),
			namespace:      envOrDefault("OHC_AGENT_K8S_NAMESPACE", defaultK8sNamespace),
			serviceAccount: os.Getenv("OHC_AGENT_K8S_SERVICE_ACCOUNT"),
			workspaceClaim: os.Getenv("OHC_AGENT_WORKSPACE_CLAIM"),
			k8sWorkDir:     envOrDefault("OHC_AGENT_K8S_WORKDIR", defaultK8sWorkDir),
			hubAddress:     resolveHubGRPCAddress(),
			builtinAddress: builtinclient.AddressFromEnv(),
			lookPath:       exec.LookPath,
			environ:        os.Environ(),
			inCluster:      os.Getenv("KUBERNETES_SERVICE_HOST") != "",
			k8sHost:        os.Getenv("KUBERNETES_SERVICE_HOST"),
			k8sPort:        firstNonEmpty(os.Getenv("KUBERNETES_SERVICE_PORT_HTTPS"), os.Getenv("KUBERNETES_SERVICE_PORT"), defaultK8sHTTPSPort),
			k8sCAPath:      defaultK8sCAPath,
		},
	}
}

func IsManagedBuiltin(agent orchestration.Agent) bool {
	return agent.Managed && isBuiltinProvider(agent.ProviderType)
}

func defaultManagedRuntimeRegion() string {
	runtime := strings.ToLower(strings.TrimSpace(os.Getenv("OHC_AGENT_RUNTIME")))
	switch runtime {
	case "kubernetes", "k8s":
		return "kubernetes"
	case "":
		if os.Getenv("KUBERNETES_SERVICE_HOST") != "" {
			return "kubernetes"
		}
	}
	return "process"
}

func (c *workerController) EnsureProvisioned(ctx context.Context, agent orchestration.Agent) error {
	if !IsManagedBuiltin(agent) {
		return nil
	}

	c.mu.Lock()
	handle, ok := c.handles[agent.ID]
	if ok && handle.Alive() {
		c.mu.Unlock()
		return nil
	}
	delete(c.handles, agent.ID)
	c.mu.Unlock()

	newHandle, err := c.provision(ctx, agent)
	if err != nil {
		return err
	}

	c.mu.Lock()
	c.handles[agent.ID] = newHandle
	c.mu.Unlock()
	return nil
}

func (c *workerController) Deprovision(ctx context.Context, agentID string) error {
	c.mu.Lock()
	handle := c.handles[agentID]
	delete(c.handles, agentID)
	c.mu.Unlock()
	if handle == nil {
		return nil
	}
	return handle.Stop(ctx)
}

func (c *workerController) provision(ctx context.Context, agent orchestration.Agent) (managedWorker, error) {
	if c.opts.runtime == "kubernetes" {
		return c.provisionKubernetes(ctx, agent)
	}
	if hubReachable(c.opts.hubAddress) {
		return c.provisionProcess(agent)
	}
	if c.hub != nil {
		slog.Info("worker controller: hub gRPC unavailable, falling back to inline builtin runner", "agent_id", agent.ID)
		return c.provisionInline(agent), nil
	}
	return nil, fmt.Errorf("hub gRPC address %q is not reachable", c.opts.hubAddress)
}

func (c *workerController) provisionInline(agent orchestration.Agent) managedWorker {
	runCtx, cancel := context.WithCancel(context.Background())
	done := make(chan struct{})
	runner := builtin.NewRunner(
		builtin.NewOrchestrationHubAdapter(c.hub),
		builtin.HubAgent{
			ID:             agent.ID,
			Name:           agent.Name,
			Role:           agent.Role,
			OrganizationID: agent.OrganizationID,
			ProviderType:   agent.ProviderType,
			Region:         agent.Region,
			Managed:        true,
		},
		c.opts.builtinAddress,
	)
	go func() {
		defer close(done)
		runner.Start(runCtx)
	}()
	return &inlineWorkerHandle{cancel: cancel, done: done, alive: true}
}

func (c *workerController) provisionProcess(agent orchestration.Agent) (managedWorker, error) {
	payload, err := encodeWorkerConfig(workerProcessConfig{
		AgentID:        agent.ID,
		AgentName:      agent.Name,
		Role:           agent.Role,
		OrganizationID: agent.OrganizationID,
		ProviderType:   agent.ProviderType,
		Region:         agent.Region,
		HubAddress:     c.opts.hubAddress,
		BuiltinAddress: c.opts.builtinAddress,
	})
	if err != nil {
		return nil, err
	}

	cmd := exec.Command(c.opts.workerBinary, "--worker-config-base64="+payload)
	cmd.Env = c.opts.environ
	cmd.Dir = defaultAgentWorkDir()
	cmd.Stdout = io.Discard
	cmd.Stderr = io.Discard

	if err := cmd.Start(); err != nil {
		return nil, fmt.Errorf("start worker process for %s: %w", agent.ID, err)
	}

	handle := &processWorkerHandle{cmd: cmd, done: make(chan struct{}), alive: true}
	go func() {
		_ = cmd.Wait()
		handle.mu.Lock()
		handle.alive = false
		handle.mu.Unlock()
		close(handle.done)
	}()
	return handle, nil
}

func (c *workerController) provisionKubernetes(ctx context.Context, agent orchestration.Agent) (managedWorker, error) {
	payload, err := encodeWorkerConfig(workerProcessConfig{
		AgentID:        agent.ID,
		AgentName:      agent.Name,
		Role:           agent.Role,
		OrganizationID: agent.OrganizationID,
		ProviderType:   agent.ProviderType,
		Region:         agent.Region,
		HubAddress:     c.opts.hubAddress,
		BuiltinAddress: "127.0.0.1:50051",
	})
	if err != nil {
		return nil, err
	}

	manifest, name, err := c.workerDeploymentManifest(agent, payload)
	if err != nil {
		return nil, err
	}

	if kubectlPath, ok := c.lookupBinary(defaultKubectlBinary); ok {
		cmd := exec.CommandContext(ctx, kubectlPath, "apply", "-f", "-")
		cmd.Env = c.opts.environ
		cmd.Stdin = bytes.NewReader(manifest)
		output, err := cmd.CombinedOutput()
		if err != nil {
			return nil, fmt.Errorf("apply worker deployment: %w: %s", err, strings.TrimSpace(string(output)))
		}
		return &kubernetesWorkerHandle{name: name, opts: c.opts, alive: true}, nil
	}

	if !c.opts.inCluster || c.opts.k8sHost == "" {
		return nil, fmt.Errorf("kubectl unavailable and in-cluster kubernetes api is not configured")
	}
	if err := c.applyDeploymentWithAPI(ctx, manifest); err != nil {
		return nil, err
	}
	return &kubernetesWorkerHandle{name: name, opts: c.opts, alive: true}, nil
}

func (c *workerController) workerDeploymentManifest(agent orchestration.Agent, payload string) ([]byte, string, error) {
	name := workerDeploymentName(agent.ID)
	workerEnv := envMaps(forwardedEnv(c.opts.environ))
	workerEnv = append(workerEnv, map[string]string{"name": "OHC_BUILTIN_AGENT_ADDRESS", "value": "127.0.0.1:50051"})

	workerContainer := map[string]interface{}{
		"name":    "worker",
		"image":   c.opts.image,
		"command": []string{"/usr/local/bin/ohc-agent-task"},
		"args":    []string{"--worker-config-base64=" + payload},
		"env":     workerEnv,
	}
	builtinContainer := map[string]interface{}{
		"name":    "builtin-agent",
		"image":   c.opts.image,
		"command": []string{"/usr/local/bin/ohc-builtin-agent"},
		"args":    builtinAgentArgs(),
		"env":     envMaps(forwardedEnv(c.opts.environ)),
	}

	podSpec := map[string]interface{}{
		"containers": []map[string]interface{}{workerContainer, builtinContainer},
	}
	if c.opts.serviceAccount != "" {
		podSpec["serviceAccountName"] = c.opts.serviceAccount
	}
	if c.opts.k8sWorkDir != "" {
		volumeMount := map[string]interface{}{"name": "workspace", "mountPath": c.opts.k8sWorkDir}
		workerContainer["workingDir"] = c.opts.k8sWorkDir
		workerContainer["volumeMounts"] = []map[string]interface{}{volumeMount}
		builtinContainer["volumeMounts"] = []map[string]interface{}{volumeMount}
		volume := map[string]interface{}{"name": "workspace"}
		if c.opts.workspaceClaim != "" {
			volume["persistentVolumeClaim"] = map[string]interface{}{"claimName": c.opts.workspaceClaim}
		} else {
			volume["emptyDir"] = map[string]interface{}{}
		}
		podSpec["volumes"] = []map[string]interface{}{volume}
	}

	manifest := map[string]interface{}{
		"apiVersion": "apps/v1",
		"kind":       "Deployment",
		"metadata": map[string]interface{}{
			"name":      name,
			"namespace": c.opts.namespace,
			"labels": map[string]string{
				"app.kubernetes.io/name":       "ohc-agent-worker",
				"app.kubernetes.io/managed-by": "ohc",
				"ohc.agent/id":                 sanitizeLabel(agent.ID),
			},
		},
		"spec": map[string]interface{}{
			"replicas": 1,
			"selector": map[string]interface{}{
				"matchLabels": map[string]string{
					"app.kubernetes.io/name": "ohc-agent-worker",
					"ohc.agent/id":           sanitizeLabel(agent.ID),
				},
			},
			"template": map[string]interface{}{
				"metadata": map[string]interface{}{
					"labels": map[string]string{
						"app.kubernetes.io/name": "ohc-agent-worker",
						"ohc.agent/id":           sanitizeLabel(agent.ID),
					},
				},
				"spec": podSpec,
			},
		},
	}

	raw, err := json.Marshal(manifest)
	if err != nil {
		return nil, "", fmt.Errorf("marshal worker deployment: %w", err)
	}
	return raw, name, nil
}

func (c *workerController) applyDeploymentWithAPI(ctx context.Context, manifest []byte) error {
	endpoint := fmt.Sprintf("https://%s:%s/apis/apps/v1/namespaces/%s/deployments", c.opts.k8sHost, c.opts.k8sPort, url.PathEscape(c.opts.namespace))
	token, err := kubernetesToken()
	if err != nil {
		return err
	}
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, endpoint, bytes.NewReader(manifest))
	if err != nil {
		return fmt.Errorf("create kubernetes deployment request: %w", err)
	}
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, err := c.doHTTPRequest(req)
	if err != nil {
		return fmt.Errorf("create worker deployment with kubernetes api: %w", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode == http.StatusConflict || (resp.StatusCode >= http.StatusOK && resp.StatusCode < http.StatusMultipleChoices) {
		return nil
	}
	body, _ := io.ReadAll(io.LimitReader(resp.Body, 16<<10))
	return fmt.Errorf("create worker deployment with kubernetes api: %s: %s", resp.Status, strings.TrimSpace(string(body)))
}

func (c *workerController) deleteDeployment(ctx context.Context, name string) error {
	if kubectlPath, ok := c.lookupBinary(defaultKubectlBinary); ok {
		cmd := exec.CommandContext(ctx, kubectlPath, "delete", "deployment", name, "--namespace", c.opts.namespace, "--ignore-not-found=true")
		cmd.Env = c.opts.environ
		output, err := cmd.CombinedOutput()
		if err != nil {
			return fmt.Errorf("delete worker deployment: %w: %s", err, strings.TrimSpace(string(output)))
		}
		return nil
	}
	if !c.opts.inCluster || c.opts.k8sHost == "" {
		return nil
	}
	endpoint := fmt.Sprintf("https://%s:%s/apis/apps/v1/namespaces/%s/deployments/%s", c.opts.k8sHost, c.opts.k8sPort, url.PathEscape(c.opts.namespace), url.PathEscape(name))
	token, err := kubernetesToken()
	if err != nil {
		return err
	}
	req, err := http.NewRequestWithContext(ctx, http.MethodDelete, endpoint, nil)
	if err != nil {
		return fmt.Errorf("create kubernetes delete request: %w", err)
	}
	req.Header.Set("Authorization", "Bearer "+token)
	resp, err := c.doHTTPRequest(req)
	if err != nil {
		return fmt.Errorf("delete worker deployment with kubernetes api: %w", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode == http.StatusNotFound || (resp.StatusCode >= http.StatusOK && resp.StatusCode < http.StatusMultipleChoices) {
		return nil
	}
	body, _ := io.ReadAll(io.LimitReader(resp.Body, 16<<10))
	return fmt.Errorf("delete worker deployment with kubernetes api: %s: %s", resp.Status, strings.TrimSpace(string(body)))
}

func (c *workerController) lookupBinary(name string) (string, bool) {
	if c.opts.lookPath == nil {
		return name, false
	}
	path, err := c.opts.lookPath(name)
	return path, err == nil
}

func (c *workerController) doHTTPRequest(req *http.Request) (*http.Response, error) {
	if c.opts.httpDo != nil {
		return c.opts.httpDo(req)
	}
	client, err := kubernetesHTTPClient(c.opts.k8sCAPath)
	if err != nil {
		return nil, err
	}
	return client.Do(req)
}

func (h *inlineWorkerHandle) Alive() bool {
	h.mu.Lock()
	defer h.mu.Unlock()
	return h.alive
}

func (h *inlineWorkerHandle) Stop(ctx context.Context) error {
	h.mu.Lock()
	if !h.alive {
		h.mu.Unlock()
		return nil
	}
	h.alive = false
	h.mu.Unlock()

	h.cancel()
	select {
	case <-h.done:
		return nil
	case <-ctx.Done():
		return ctx.Err()
	}
}

func (h *processWorkerHandle) Alive() bool {
	h.mu.Lock()
	defer h.mu.Unlock()
	return h.alive
}

func (h *processWorkerHandle) Stop(ctx context.Context) error {
	h.mu.Lock()
	if !h.alive {
		h.mu.Unlock()
		return nil
	}
	h.alive = false
	proc := h.cmd.Process
	h.mu.Unlock()

	if proc != nil {
		_ = proc.Kill()
	}
	select {
	case <-h.done:
		return nil
	case <-ctx.Done():
		return ctx.Err()
	}
}

func (h *kubernetesWorkerHandle) Alive() bool {
	h.mu.Lock()
	defer h.mu.Unlock()
	return h.alive
}

func (h *kubernetesWorkerHandle) Stop(ctx context.Context) error {
	h.mu.Lock()
	if !h.alive {
		h.mu.Unlock()
		return nil
	}
	h.alive = false
	h.mu.Unlock()

	controller := &workerController{opts: h.opts}
	return controller.deleteDeployment(ctx, h.name)
}

func encodeWorkerConfig(cfg workerProcessConfig) (string, error) {
	raw, err := json.Marshal(cfg)
	if err != nil {
		return "", fmt.Errorf("marshal worker config: %w", err)
	}
	return base64.StdEncoding.EncodeToString(raw), nil
}

func resolveHubGRPCAddress() string {
	if address := os.Getenv("OHC_HUB_GRPC_ADDRESS"); address != "" {
		return address
	}
	grpcPort := strings.TrimSpace(os.Getenv("GRPC_PORT"))
	if grpcPort == "" {
		return defaultHubGRPCAddress
	}
	if strings.HasPrefix(grpcPort, ":") {
		return "127.0.0.1" + grpcPort
	}
	host, port, err := net.SplitHostPort(grpcPort)
	if err == nil {
		if host == "" || host == "0.0.0.0" || host == "::" || host == "[::]" {
			host = "127.0.0.1"
		}
		return net.JoinHostPort(host, port)
	}
	if strings.Count(grpcPort, ":") == 1 {
		parts := strings.SplitN(grpcPort, ":", 2)
		host = parts[0]
		if host == "" || host == "0.0.0.0" {
			host = "127.0.0.1"
		}
		return net.JoinHostPort(host, parts[1])
	}
	return grpcPort
}

func hubReachable(address string) bool {
	conn, err := net.DialTimeout("tcp", address, 250*time.Millisecond)
	if err != nil {
		return false
	}
	_ = conn.Close()
	return true
}

func workerDeploymentName(agentID string) string {
	return "ohc-agent-worker-" + shortHash(agentID)
}

func defaultWorkerBinaryPath() string {
	if exe, err := os.Executable(); err == nil {
		candidate := filepath.Join(filepath.Dir(exe), defaultWorkerBinaryName)
		if _, statErr := os.Stat(candidate); statErr == nil {
			return candidate
		}
	}
	return defaultWorkerBinaryName
}

func envOrDefault(key, fallback string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return fallback
}

func isBuiltinProvider(providerType string) bool {
	return providerType == "" || providerType == string(ProviderTypeBuiltin)
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

func builtinAgentArgs() []string {
	args := []string{"--port=50051"}
	if provider := os.Getenv("OHC_BUILTIN_AGENT_PROVIDER"); provider != "" {
		args = append(args, "--llm_provider="+provider)
	}
	if model := os.Getenv("OHC_BUILTIN_AGENT_MODEL"); model != "" {
		args = append(args, "--model="+model)
	}
	if endpoint := os.Getenv("OHC_BUILTIN_AGENT_ENDPOINT"); endpoint != "" {
		args = append(args, "--llm_endpoint="+endpoint)
	}
	if prompt := os.Getenv("OHC_BUILTIN_AGENT_SYSTEM_PROMPT"); prompt != "" {
		args = append(args, "--system_prompt="+prompt)
	}
	if maxTokens := os.Getenv("OHC_BUILTIN_AGENT_MAX_TOKENS"); maxTokens != "" {
		args = append(args, "--max_tokens="+maxTokens)
	}
	if temperature := os.Getenv("OHC_BUILTIN_AGENT_TEMPERATURE"); temperature != "" {
		args = append(args, "--temperature="+temperature)
	}
	if maxIterations := os.Getenv("OHC_BUILTIN_AGENT_MAX_ITERATIONS"); maxIterations != "" {
		args = append(args, "--max_iterations="+maxIterations)
	}
	if maxContext := os.Getenv("OHC_BUILTIN_AGENT_MAX_CONTEXT_MESSAGES"); maxContext != "" {
		args = append(args, "--max_context_messages="+maxContext)
	}
	return args
}

func sanitizeLabel(value string) string {
	value = strings.ToLower(value)
	b := strings.Builder{}
	for _, r := range value {
		switch {
		case r >= 'a' && r <= 'z':
			b.WriteRune(r)
		case r >= '0' && r <= '9':
			b.WriteRune(r)
		case r == '-' || r == '.':
			b.WriteRune(r)
		default:
			b.WriteRune('-')
		}
	}
	cleaned := strings.Trim(b.String(), "-.")
	if cleaned == "" {
		return "agent"
	}
	if len(cleaned) > 63 {
		return cleaned[:63]
	}
	return cleaned
}

func kubernetesToken() (string, error) {
	raw, err := os.ReadFile(defaultK8sTokenPath)
	if err != nil {
		return "", fmt.Errorf("read kubernetes service account token: %w", err)
	}
	return strings.TrimSpace(string(raw)), nil
}

func kubernetesHTTPClient(caPath string) (*http.Client, error) {
	caPEM, err := os.ReadFile(caPath)
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

	return &http.Client{Timeout: 30 * time.Second, Transport: clone}, nil
}

func shortHash(input string) string {
	hash := fnv.New32a()
	_, _ = hash.Write([]byte(input))
	return fmt.Sprintf("%08x", hash.Sum32())
}

func firstNonEmpty(values ...string) string {
	for _, value := range values {
		if value != "" {
			return value
		}
	}
	return ""
}
