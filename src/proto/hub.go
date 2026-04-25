// Package pb contains hand-written Go stubs matching hub.proto and related
// proto definitions. These stubs implement the same API shape as protoc-generated
// code so that application code compiles without running protoc.
//
// The "builder" pattern mirrors proto edition 2024's opaque API:
//   msg := pb.Message_builder{Id: proto.String("x")}.Build()
//   msg.GetId()  // => "x"
package pb

import (
	"context"

	grpc "google.golang.org/grpc"
)

// ── Agent ──────────────────────────────────────────────────────────────────

// Agent represents a registered agent.
type Agent struct {
	id             *string
	name           *string
	role           *string
	organizationId *string
	status         *string
	providerType   *string
}

func (a *Agent) GetId() string {
	if a == nil || a.id == nil {
		return ""
	}
	return *a.id
}

func (a *Agent) GetName() string {
	if a == nil || a.name == nil {
		return ""
	}
	return *a.name
}

func (a *Agent) GetRole() string {
	if a == nil || a.role == nil {
		return ""
	}
	return *a.role
}

func (a *Agent) GetOrganizationId() string {
	if a == nil || a.organizationId == nil {
		return ""
	}
	return *a.organizationId
}

func (a *Agent) GetStatus() string {
	if a == nil || a.status == nil {
		return ""
	}
	return *a.status
}

func (a *Agent) GetProviderType() string {
	if a == nil || a.providerType == nil {
		return ""
	}
	return *a.providerType
}

// Agent_builder constructs an Agent.
type Agent_builder struct {
	Id             *string
	Name           *string
	Role           *string
	OrganizationId *string
	Status         *string
	ProviderType   *string
}

func (b Agent_builder) Build() *Agent {
	return &Agent{
		id:             b.Id,
		name:           b.Name,
		role:           b.Role,
		organizationId: b.OrganizationId,
		status:         b.Status,
		providerType:   b.ProviderType,
	}
}

// ── Message ────────────────────────────────────────────────────────────────

// Message represents a message between agents.
type Message struct {
	id             *string
	fromAgent      *string
	toAgent        *string
	typ            *string
	content        *string
	meetingId      *string
	occurredAtUnix *int64
}

func (m *Message) GetId() string {
	if m == nil || m.id == nil {
		return ""
	}
	return *m.id
}

func (m *Message) GetFromAgent() string {
	if m == nil || m.fromAgent == nil {
		return ""
	}
	return *m.fromAgent
}

func (m *Message) GetToAgent() string {
	if m == nil || m.toAgent == nil {
		return ""
	}
	return *m.toAgent
}

func (m *Message) GetType() string {
	if m == nil || m.typ == nil {
		return ""
	}
	return *m.typ
}

func (m *Message) GetContent() string {
	if m == nil || m.content == nil {
		return ""
	}
	return *m.content
}

func (m *Message) GetMeetingId() string {
	if m == nil || m.meetingId == nil {
		return ""
	}
	return *m.meetingId
}

func (m *Message) GetOccurredAtUnix() int64 {
	if m == nil || m.occurredAtUnix == nil {
		return 0
	}
	return *m.occurredAtUnix
}

// Message_builder constructs a Message.
type Message_builder struct {
	Id             *string
	FromAgent      *string
	ToAgent        *string
	Type           *string
	Content        *string
	MeetingId      *string
	OccurredAtUnix *int64
}

func (b Message_builder) Build() *Message {
	return &Message{
		id:             b.Id,
		fromAgent:      b.FromAgent,
		toAgent:        b.ToAgent,
		typ:            b.Type,
		content:        b.Content,
		meetingId:      b.MeetingId,
		occurredAtUnix: b.OccurredAtUnix,
	}
}

// ── MeetingRoom ────────────────────────────────────────────────────────────

// MeetingRoom represents a multi-agent meeting room.
type MeetingRoom struct {
	id           *string
	agenda       *string
	participants []string
	transcript   []*Message
}

func (r *MeetingRoom) GetId() string {
	if r == nil || r.id == nil {
		return ""
	}
	return *r.id
}

func (r *MeetingRoom) GetAgenda() string {
	if r == nil || r.agenda == nil {
		return ""
	}
	return *r.agenda
}

func (r *MeetingRoom) GetParticipants() []string {
	if r == nil {
		return nil
	}
	return r.participants
}

func (r *MeetingRoom) GetTranscript() []*Message {
	if r == nil {
		return nil
	}
	return r.transcript
}

// MeetingRoom_builder constructs a MeetingRoom.
type MeetingRoom_builder struct {
	Id           *string
	Agenda       *string
	Participants []string
	Transcript   []*Message
}

func (b MeetingRoom_builder) Build() *MeetingRoom {
	return &MeetingRoom{
		id:           b.Id,
		agenda:       b.Agenda,
		participants: b.Participants,
		transcript:   b.Transcript,
	}
}

// ── RegisterAgent ──────────────────────────────────────────────────────────

// RegisterAgentRequest is the input for RegisterAgent RPC.
type RegisterAgentRequest struct {
	agent *Agent
}

func (r *RegisterAgentRequest) GetAgent() *Agent {
	if r == nil {
		return nil
	}
	return r.agent
}

// RegisterAgentRequest_builder constructs a RegisterAgentRequest.
type RegisterAgentRequest_builder struct {
	Agent *Agent
}

func (b RegisterAgentRequest_builder) Build() *RegisterAgentRequest {
	return &RegisterAgentRequest{agent: b.Agent}
}

// RegisterAgentResponse is the output for RegisterAgent RPC.
type RegisterAgentResponse struct {
	success *bool
}

func (r *RegisterAgentResponse) GetSuccess() bool {
	if r == nil || r.success == nil {
		return false
	}
	return *r.success
}

// RegisterAgentResponse_builder constructs a RegisterAgentResponse.
type RegisterAgentResponse_builder struct {
	Success *bool
}

func (b RegisterAgentResponse_builder) Build() *RegisterAgentResponse {
	return &RegisterAgentResponse{success: b.Success}
}

// ── OpenMeeting ────────────────────────────────────────────────────────────

// OpenMeetingRequest is the input for OpenMeeting RPC.
type OpenMeetingRequest struct {
	meetingId    *string
	agenda       *string
	participants []string
}

func (r *OpenMeetingRequest) GetMeetingId() string {
	if r == nil || r.meetingId == nil {
		return ""
	}
	return *r.meetingId
}

func (r *OpenMeetingRequest) GetAgenda() string {
	if r == nil || r.agenda == nil {
		return ""
	}
	return *r.agenda
}

func (r *OpenMeetingRequest) GetParticipants() []string {
	if r == nil {
		return nil
	}
	return r.participants
}

// OpenMeetingRequest_builder constructs an OpenMeetingRequest.
type OpenMeetingRequest_builder struct {
	MeetingId    *string
	Agenda       *string
	Participants []string
}

func (b OpenMeetingRequest_builder) Build() *OpenMeetingRequest {
	return &OpenMeetingRequest{
		meetingId:    b.MeetingId,
		agenda:       b.Agenda,
		participants: b.Participants,
	}
}

// ── PublishMessage ─────────────────────────────────────────────────────────

// PublishMessageRequest is the input for Publish RPC.
type PublishMessageRequest struct {
	message *Message
}

func (r *PublishMessageRequest) GetMessage() *Message {
	if r == nil {
		return nil
	}
	return r.message
}

// PublishMessageRequest_builder constructs a PublishMessageRequest.
type PublishMessageRequest_builder struct {
	Message *Message
}

func (b PublishMessageRequest_builder) Build() *PublishMessageRequest {
	return &PublishMessageRequest{message: b.Message}
}

// PublishMessageResponse is the output for Publish RPC.
type PublishMessageResponse struct {
	success *bool
}

func (r *PublishMessageResponse) GetSuccess() bool {
	if r == nil || r.success == nil {
		return false
	}
	return *r.success
}

// PublishMessageResponse_builder constructs a PublishMessageResponse.
type PublishMessageResponse_builder struct {
	Success *bool
}

func (b PublishMessageResponse_builder) Build() *PublishMessageResponse {
	return &PublishMessageResponse{success: b.Success}
}

// ── DelegateTask ───────────────────────────────────────────────────────────

// DelegateTaskRequest is the input for DelegateTask RPC.
type DelegateTaskRequest struct {
	fromAgentId *string
	toAgentId   *string
	task        *Message
}

func (r *DelegateTaskRequest) GetFromAgentId() string {
	if r == nil || r.fromAgentId == nil {
		return ""
	}
	return *r.fromAgentId
}

func (r *DelegateTaskRequest) GetToAgentId() string {
	if r == nil || r.toAgentId == nil {
		return ""
	}
	return *r.toAgentId
}

func (r *DelegateTaskRequest) GetTask() *Message {
	if r == nil {
		return nil
	}
	return r.task
}

// DelegateTaskRequest_builder constructs a DelegateTaskRequest.
type DelegateTaskRequest_builder struct {
	FromAgentId *string
	ToAgentId   *string
	Task        *Message
}

func (b DelegateTaskRequest_builder) Build() *DelegateTaskRequest {
	return &DelegateTaskRequest{
		fromAgentId: b.FromAgentId,
		toAgentId:   b.ToAgentId,
		task:        b.Task,
	}
}

// DelegateTaskResponse is the output for DelegateTask and DelegateSubTask RPCs.
type DelegateTaskResponse struct {
	success *bool
}

func (r *DelegateTaskResponse) GetSuccess() bool {
	if r == nil || r.success == nil {
		return false
	}
	return *r.success
}

// DelegateTaskResponse_builder constructs a DelegateTaskResponse.
type DelegateTaskResponse_builder struct {
	Success *bool
}

func (b DelegateTaskResponse_builder) Build() *DelegateTaskResponse {
	return &DelegateTaskResponse{success: b.Success}
}

// ── SubTask ────────────────────────────────────────────────────────────────

// SubTask is the input for DelegateSubTask RPC.
type SubTask struct {
	taskId        *string
	targetRole    *string
	instruction   *string
	parentThreadId *string
	fromAgentId   *string
}

func (s *SubTask) GetTaskId() string {
	if s == nil || s.taskId == nil {
		return ""
	}
	return *s.taskId
}

func (s *SubTask) GetTargetRole() string {
	if s == nil || s.targetRole == nil {
		return ""
	}
	return *s.targetRole
}

func (s *SubTask) GetInstruction() string {
	if s == nil || s.instruction == nil {
		return ""
	}
	return *s.instruction
}

func (s *SubTask) GetParentThreadId() string {
	if s == nil || s.parentThreadId == nil {
		return ""
	}
	return *s.parentThreadId
}

func (s *SubTask) GetFromAgentId() string {
	if s == nil || s.fromAgentId == nil {
		return ""
	}
	return *s.fromAgentId
}

// SubTask_builder constructs a SubTask.
type SubTask_builder struct {
	TaskId         *string
	TargetRole     *string
	Instruction    *string
	ParentThreadId *string
	FromAgentId    *string
}

func (b SubTask_builder) Build() *SubTask {
	return &SubTask{
		taskId:         b.TaskId,
		targetRole:     b.TargetRole,
		instruction:    b.Instruction,
		parentThreadId: b.ParentThreadId,
		fromAgentId:    b.FromAgentId,
	}
}

// ── StreamMessages ─────────────────────────────────────────────────────────

// StreamMessagesRequest is the input for StreamMessages RPC.
type StreamMessagesRequest struct {
	agentId *string
}

func (r *StreamMessagesRequest) GetAgentId() string {
	if r == nil || r.agentId == nil {
		return ""
	}
	return *r.agentId
}

// StreamMessagesRequest_builder constructs a StreamMessagesRequest.
type StreamMessagesRequest_builder struct {
	AgentId *string
}

func (b StreamMessagesRequest_builder) Build() *StreamMessagesRequest {
	return &StreamMessagesRequest{agentId: b.AgentId}
}

// ── Reason ─────────────────────────────────────────────────────────────────

// ReasonRequest is the input for Reason RPC.
type ReasonRequest struct {
	prompt      *string
	fromAgentId *string
}

func (r *ReasonRequest) GetPrompt() string {
	if r == nil || r.prompt == nil {
		return ""
	}
	return *r.prompt
}

func (r *ReasonRequest) GetFromAgentId() string {
	if r == nil || r.fromAgentId == nil {
		return ""
	}
	return *r.fromAgentId
}

// ReasonRequest_builder constructs a ReasonRequest.
type ReasonRequest_builder struct {
	Prompt      *string
	FromAgentId *string
}

func (b ReasonRequest_builder) Build() *ReasonRequest {
	return &ReasonRequest{
		prompt:      b.Prompt,
		fromAgentId: b.FromAgentId,
	}
}

// ReasonResponse is the output for Reason RPC.
type ReasonResponse struct {
	content *string
}

func (r *ReasonResponse) GetContent() string {
	if r == nil || r.content == nil {
		return ""
	}
	return *r.content
}

// ReasonResponse_builder constructs a ReasonResponse.
type ReasonResponse_builder struct {
	Content *string
}

func (b ReasonResponse_builder) Build() *ReasonResponse {
	return &ReasonResponse{content: b.Content}
}

// ── AgentCapabilities ──────────────────────────────────────────────────────

// AgentCapabilities describes capabilities advertised by an agent on the mesh.
type AgentCapabilities struct {
	AgentId            string
	SupportedSkills    []string
	MaxConcurrentTasks int32
}

func (c *AgentCapabilities) GetAgentId() string {
	if c == nil {
		return ""
	}
	return c.AgentId
}

func (c *AgentCapabilities) GetSupportedSkills() []string {
	if c == nil {
		return nil
	}
	return c.SupportedSkills
}

func (c *AgentCapabilities) GetMaxConcurrentTasks() int32 {
	if c == nil {
		return 0
	}
	return c.MaxConcurrentTasks
}

// ── Query ──────────────────────────────────────────────────────────────────

// Query is used for DiscoverAgents RPC.
type Query struct {
	Filter string
}

func (q *Query) GetFilter() string {
	if q == nil {
		return ""
	}
	return q.Filter
}

// ── EventStreamRequest ─────────────────────────────────────────────────────

// EventStreamRequest is the input for StreamMeshEvents RPC.
type EventStreamRequest struct {
	Topic string
}

func (r *EventStreamRequest) GetTopic() string {
	if r == nil {
		return ""
	}
	return r.Topic
}

// ── MeshEvent ──────────────────────────────────────────────────────────────

// MeshEvent is an event streamed from the mesh.
type MeshEvent struct {
	eventId   *string
	topic     *string
	payload   []byte
	timestamp *int64
}

func (e *MeshEvent) GetEventId() string {
	if e == nil || e.eventId == nil {
		return ""
	}
	return *e.eventId
}

func (e *MeshEvent) GetTopic() string {
	if e == nil || e.topic == nil {
		return ""
	}
	return *e.topic
}

func (e *MeshEvent) GetPayload() []byte {
	if e == nil {
		return nil
	}
	return e.payload
}

func (e *MeshEvent) GetTimestamp() int64 {
	if e == nil || e.timestamp == nil {
		return 0
	}
	return *e.timestamp
}

// MeshEvent_builder constructs a MeshEvent.
type MeshEvent_builder struct {
	EventId   *string
	Topic     *string
	Payload   []byte
	Timestamp *int64
}

func (b MeshEvent_builder) Build() *MeshEvent {
	return &MeshEvent{
		eventId:   b.EventId,
		topic:     b.Topic,
		payload:   b.Payload,
		timestamp: b.Timestamp,
	}
}

// ── ToolParameterAutoCorrectionEvent ──────────────────────────────────────

// ToolParameterAutoCorrectionEvent records an automatic tool parameter correction.
type ToolParameterAutoCorrectionEvent struct {
	eventId *string
	agentId *string
	payload []byte
}

func (e *ToolParameterAutoCorrectionEvent) GetEventId() string {
	if e == nil || e.eventId == nil {
		return ""
	}
	return *e.eventId
}

func (e *ToolParameterAutoCorrectionEvent) GetAgentId() string {
	if e == nil || e.agentId == nil {
		return ""
	}
	return *e.agentId
}

func (e *ToolParameterAutoCorrectionEvent) GetPayload() []byte {
	if e == nil {
		return nil
	}
	return e.payload
}

// ToolParameterAutoCorrectionEvent_builder constructs a ToolParameterAutoCorrectionEvent.
type ToolParameterAutoCorrectionEvent_builder struct {
	EventId *string
	AgentId *string
	Payload []byte
}

func (b ToolParameterAutoCorrectionEvent_builder) Build() *ToolParameterAutoCorrectionEvent {
	return &ToolParameterAutoCorrectionEvent{
		eventId: b.EventId,
		agentId: b.AgentId,
		payload: b.Payload,
	}
}

// ── TokenEfficientContextSummarizationEvent ────────────────────────────────

// TokenEfficientContextSummarizationEvent records context summarization telemetry.
type TokenEfficientContextSummarizationEvent struct {
	EventId string
	AgentId string
	Payload []byte
}

func (e *TokenEfficientContextSummarizationEvent) GetEventId() string {
	if e == nil {
		return ""
	}
	return e.EventId
}

func (e *TokenEfficientContextSummarizationEvent) GetAgentId() string {
	if e == nil {
		return ""
	}
	return e.AgentId
}

func (e *TokenEfficientContextSummarizationEvent) GetPayload() []byte {
	if e == nil {
		return nil
	}
	return e.Payload
}

// ── WizardField / WizardStep / IntegrationMetadata ────────────────────────

// WizardField describes a single field in an integration setup wizard step.
type WizardField struct {
	key         *string
	label       *string
	typ         *string
	required    *bool
	description *string
}

func (f *WizardField) GetKey() string {
	if f == nil || f.key == nil {
		return ""
	}
	return *f.key
}

func (f *WizardField) GetLabel() string {
	if f == nil || f.label == nil {
		return ""
	}
	return *f.label
}

func (f *WizardField) GetType() string {
	if f == nil || f.typ == nil {
		return ""
	}
	return *f.typ
}

func (f *WizardField) GetRequired() bool {
	if f == nil || f.required == nil {
		return false
	}
	return *f.required
}

func (f *WizardField) GetDescription() string {
	if f == nil || f.description == nil {
		return ""
	}
	return *f.description
}

// WizardField_builder constructs a WizardField.
type WizardField_builder struct {
	Key         *string
	Label       *string
	Type        *string
	Required    *bool
	Description *string
}

func (b WizardField_builder) Build() *WizardField {
	return &WizardField{
		key:         b.Key,
		label:       b.Label,
		typ:         b.Type,
		required:    b.Required,
		description: b.Description,
	}
}

// WizardStep describes one step in an integration setup wizard.
type WizardStep struct {
	title       *string
	description *string
	fields      []*WizardField
}

func (s *WizardStep) GetTitle() string {
	if s == nil || s.title == nil {
		return ""
	}
	return *s.title
}

func (s *WizardStep) GetDescription() string {
	if s == nil || s.description == nil {
		return ""
	}
	return *s.description
}

func (s *WizardStep) GetFields() []*WizardField {
	if s == nil {
		return nil
	}
	return s.fields
}

// WizardStep_builder constructs a WizardStep.
type WizardStep_builder struct {
	Title       *string
	Description *string
	Fields      []*WizardField
}

func (b WizardStep_builder) Build() *WizardStep {
	return &WizardStep{
		title:       b.Title,
		description: b.Description,
		fields:      b.Fields,
	}
}

// IntegrationMetadata describes a third-party integration provider.
type IntegrationMetadata struct {
	id          *string
	name        *string
	typ         *string
	category    *string
	baseUrl     *string
	description *string
	publisher   *string
	icon        *string
	tags        []string
}

func (m *IntegrationMetadata) GetId() string {
	if m == nil || m.id == nil {
		return ""
	}
	return *m.id
}

func (m *IntegrationMetadata) GetName() string {
	if m == nil || m.name == nil {
		return ""
	}
	return *m.name
}

func (m *IntegrationMetadata) GetType() string {
	if m == nil || m.typ == nil {
		return ""
	}
	return *m.typ
}

func (m *IntegrationMetadata) GetCategory() string {
	if m == nil || m.category == nil {
		return ""
	}
	return *m.category
}

func (m *IntegrationMetadata) GetBaseUrl() string {
	if m == nil || m.baseUrl == nil {
		return ""
	}
	return *m.baseUrl
}

func (m *IntegrationMetadata) GetDescription() string {
	if m == nil || m.description == nil {
		return ""
	}
	return *m.description
}

func (m *IntegrationMetadata) GetPublisher() string {
	if m == nil || m.publisher == nil {
		return ""
	}
	return *m.publisher
}

func (m *IntegrationMetadata) GetIcon() string {
	if m == nil || m.icon == nil {
		return ""
	}
	return *m.icon
}

func (m *IntegrationMetadata) GetTags() []string {
	if m == nil {
		return nil
	}
	return m.tags
}

// IntegrationMetadata_builder constructs an IntegrationMetadata.
type IntegrationMetadata_builder struct {
	Id          *string
	Name        *string
	Type        *string
	Category    *string
	BaseUrl     *string
	Description *string
	Publisher   *string
	Icon        *string
	Tags        []string
}

func (b IntegrationMetadata_builder) Build() *IntegrationMetadata {
	return &IntegrationMetadata{
		id:          b.Id,
		name:        b.Name,
		typ:         b.Type,
		category:    b.Category,
		baseUrl:     b.BaseUrl,
		description: b.Description,
		publisher:   b.Publisher,
		icon:        b.Icon,
		tags:        b.Tags,
	}
}

// ── gRPC streaming interfaces ─────────────────────────────────────────────

// HubService_StreamMessagesServer is the server-side streaming interface for
// the StreamMessages RPC.
type HubService_StreamMessagesServer interface {
	Send(*Message) error
	grpc.ServerStream
}

// HubService_DiscoverAgentsServer is the server-side streaming interface for
// the DiscoverAgents RPC.
type HubService_DiscoverAgentsServer interface {
	Send(*AgentCapabilities) error
	grpc.ServerStream
}

// HubService_StreamMeshEventsServer is the server-side streaming interface for
// the StreamMeshEvents RPC.
type HubService_StreamMeshEventsServer interface {
	Send(*MeshEvent) error
	grpc.ServerStream
}

// ── HubService gRPC server ─────────────────────────────────────────────────

// HubServiceServer is the interface that must be implemented by gRPC HubService servers.
type HubServiceServer interface {
	RegisterAgent(context.Context, *RegisterAgentRequest) (*RegisterAgentResponse, error)
	OpenMeeting(context.Context, *OpenMeetingRequest) (*MeetingRoom, error)
	Publish(context.Context, *PublishMessageRequest) (*PublishMessageResponse, error)
	DelegateTask(context.Context, *DelegateTaskRequest) (*DelegateTaskResponse, error)
	StreamMessages(*StreamMessagesRequest, HubService_StreamMessagesServer) error
	Reason(context.Context, *ReasonRequest) (*ReasonResponse, error)
	DelegateSubTask(context.Context, *SubTask) (*DelegateTaskResponse, error)
	AdvertiseCapabilities(context.Context, *AgentCapabilities) (*PublishMessageResponse, error)
	DiscoverAgents(*Query, HubService_DiscoverAgentsServer) error
	StreamMeshEvents(*EventStreamRequest, HubService_StreamMeshEventsServer) error
	mustEmbedUnimplementedHubServiceServer()
}

// UnimplementedHubServiceServer provides default (unimplemented) methods for
// all HubService RPC handlers. Embed this in a server struct to satisfy the
// HubServiceServer interface.
type UnimplementedHubServiceServer struct{}

func (UnimplementedHubServiceServer) RegisterAgent(context.Context, *RegisterAgentRequest) (*RegisterAgentResponse, error) {
	return nil, nil
}

func (UnimplementedHubServiceServer) OpenMeeting(context.Context, *OpenMeetingRequest) (*MeetingRoom, error) {
	return nil, nil
}

func (UnimplementedHubServiceServer) Publish(context.Context, *PublishMessageRequest) (*PublishMessageResponse, error) {
	return nil, nil
}

func (UnimplementedHubServiceServer) DelegateTask(context.Context, *DelegateTaskRequest) (*DelegateTaskResponse, error) {
	return nil, nil
}

func (UnimplementedHubServiceServer) StreamMessages(*StreamMessagesRequest, HubService_StreamMessagesServer) error {
	return nil
}

func (UnimplementedHubServiceServer) Reason(context.Context, *ReasonRequest) (*ReasonResponse, error) {
	return nil, nil
}

func (UnimplementedHubServiceServer) DelegateSubTask(context.Context, *SubTask) (*DelegateTaskResponse, error) {
	return nil, nil
}

func (UnimplementedHubServiceServer) AdvertiseCapabilities(context.Context, *AgentCapabilities) (*PublishMessageResponse, error) {
	return nil, nil
}

func (UnimplementedHubServiceServer) DiscoverAgents(*Query, HubService_DiscoverAgentsServer) error {
	return nil
}

func (UnimplementedHubServiceServer) StreamMeshEvents(*EventStreamRequest, HubService_StreamMeshEventsServer) error {
	return nil
}

func (UnimplementedHubServiceServer) mustEmbedUnimplementedHubServiceServer() {}

// RegisterHubServiceServer registers the HubService implementation with a gRPC server.
// This is a no-op stub – the full registration uses Bazel-generated code in production.
func RegisterHubServiceServer(_ *grpc.Server, _ HubServiceServer) {}
