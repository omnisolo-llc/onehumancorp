import re

with open("srcs/integrations/registry.go", "r") as f:
    code = f.read()

# 1. Update Registry struct
code = re.sub(
    r"type Registry struct \{\n\tmu\s+sync\.RWMutex\n\tintegrations \[\]Integration\n\tcredentials\s+map\[string\]IntegrationCredentials",
    "type Registry struct {\n\tmu           sync.RWMutex\n\tinstances    map[string]*IntegrationInstance\n\tcredentials  map[string]IntegrationCredentials",
    code
)

# 2. Update NewRegistry
code = re.sub(
    r"func NewRegistry\(\) \*Registry \{\n\treturn &Registry\{\n\t\tintegrations:\s*defaultIntegrations\(\),",
    "func NewRegistry() *Registry {\n\treturn &Registry{\n\t\tinstances:    make(map[string]*IntegrationInstance),",
    code
)

# 3. Update Integrations() and IntegrationsByCategory() and Integration()
code = re.sub(
    r"func \(r \*Registry\) Integrations\(\) \[\]Integration \{\n\tr\.mu\.RLock\(\)\n\tdefer r\.mu\.RUnlock\(\)\n\n\treturn append\(\[\]Integration\(nil\), r\.integrations\.\.\.\)\n\}",
    """func (r *Registry) Instances() []*IntegrationInstance {
\tr.mu.RLock()
\tdefer r.mu.RUnlock()
\tvar res []*IntegrationInstance
\tfor _, inst := range r.instances {
\t\tres = append(res, inst)
\t}
\treturn res
}""",
    code
)

code = re.sub(
    r"func \(r \*Registry\) IntegrationsByCategory\(cat Category\) \[\]Integration \{\n\tr\.mu\.RLock\(\)\n\tdefer r\.mu\.RUnlock\(\)\n\n\tvar result \[\]Integration\n\tfor _, i := range r\.integrations \{\n\t\tif i\.Category == cat \{\n\t\t\tresult = append\(result, i\)\n\t\t\}\n\t\}\n\treturn result\n\}",
    """func (r *Registry) InstancesByCategory(cat Category) []*IntegrationInstance {
\tr.mu.RLock()
\tdefer r.mu.RUnlock()
\tvar result []*IntegrationInstance
\tfor _, i := range r.instances {
\t\tif i.Category == cat {
\t\t\tresult = append(result, i)
\t\t}
\t}
\treturn result
}""",
    code
)

code = re.sub(
    r"func \(r \*Registry\) Integration\(id string\) \(Integration, bool\) \{\n\tr\.mu\.RLock\(\)\n\tdefer r\.mu\.RUnlock\(\)\n\n\tfor _, i := range r\.integrations \{\n\t\tif i\.ID == id \{\n\t\t\treturn i, true\n\t\t\}\n\t\}\n\treturn Integration\{\}, false\n\}",
    """func (r *Registry) Instance(id string) (*IntegrationInstance, bool) {
\tr.mu.RLock()
\tdefer r.mu.RUnlock()
\tinst, ok := r.instances[id]
\treturn inst, ok
}""",
    code
)

# 4. Connect
code = code.replace("func (r *Registry) Connect(id, baseURL string, creds ...IntegrationCredentials) (Integration, error) {", "func (r *Registry) Connect(id, baseURL string, creds ...IntegrationCredentials) (*IntegrationInstance, error) {")
code = code.replace("return Integration{}, err", "return nil, err")
code = code.replace("return Integration{}, errors.New(\"integration not found\")", "return nil, errors.New(\"integration not found\")")

code = re.sub(
    r"for idx, i := range r\.integrations \{\n\t\tif i\.ID == id \{\n\t\t\tr\.integrations\[idx\]\.Status = StatusConnected\n\t\t\tr\.integrations\[idx\]\.BaseURL = baseURL",
    """if inst, ok := r.instances[id]; ok {
\t\tinst.Status = StatusConnected
\t\tinst.BaseURL = baseURL""",
    code
)
code = re.sub(
    r"if !creds\[0\]\.IsEmpty\(\) \{\n\t\t\t\tr\.integrations\[idx\]\.HasCredentials = true\n\t\t\t\tcredsToSave = creds\[0\]\n\t\t\t\}",
    """if !creds[0].IsEmpty() {
\t\t\tinst.HasCredentials = true
\t\t\tcredsToSave = creds[0]
\t\t}""",
    code
)
code = re.sub(
    r"if i\.Type == IntegrationTypeGoogleChat \{\n\t\t\t\tr\.integrations\[idx\]\.Chatspace = \"spaces/\" \+ baseURL\n\t\t\t\}",
    """if inst.Type == IntegrationTypeGoogleChat {
\t\t\tinst.Chatspace = "spaces/" + baseURL
\t\t}""",
    code
)
code = re.sub(
    r"return r\.integrations\[idx\], nil\n\t\t\}\n\t\}",
    "return inst, nil\n\t}",
    code
)

# 5. Disconnect
code = code.replace("func (r *Registry) Disconnect(id string) (Integration, error) {", "func (r *Registry) Disconnect(id string) (*IntegrationInstance, error) {")
code = re.sub(
    r"for idx, i := range r\.integrations \{\n\t\tif i\.ID == id \{\n\t\t\tr\.integrations\[idx\]\.Status = StatusDisconnected\n\t\t\tdelete\(r\.credentials, id\)\n\t\t\treturn r\.integrations\[idx\], nil\n\t\t\}\n\t\}\n\treturn nil, errors\.New\(\"integration not found\"\)",
    """if inst, ok := r.instances[id]; ok {
\t\tinst.Status = StatusDisconnected
\t\tdelete(r.credentials, id)
\t\treturn inst, nil
\t}
\treturn nil, errors.New("integration not found")""",
    code
)

# 6. findIntegration
code = code.replace("func (r *Registry) findIntegration(id string) (Integration, bool) {", "func (r *Registry) findIntegration(id string) (*IntegrationInstance, bool) {")
code = re.sub(
    r"for _, i := range r\.integrations \{\n\t\tif i\.ID == id \{\n\t\t\treturn i, true\n\t\t\}\n\t\}\n\treturn Integration\{\}, false",
    """inst, ok := r.instances[id]
\treturn inst, ok""",
    code
)

# 7. Remove defaultIntegrations block completely at the end
code = re.sub(
    r"// defaultIntegrations returns the built-in set of supported external services,.*?\n\n",
    "",
    code,
    flags=re.DOTALL
)

with open("srcs/integrations/registry.go", "w") as f:
    f.write(code)

