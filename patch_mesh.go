package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	filePath := "srcs/server/orchestration/mesh.go"
	content, err := ioutil.ReadFile(filePath)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	strContent := string(content)

	oldStruct := `type LocalTeammateMesh struct {
	db                  db.Provider
	broadcast           []chan Task
	persist             []chan Task
	mu                  []sync.RWMutex
	subs                []map[chan Task]struct{}
	coordBroadcast      []chan MeshMessage
	coordSubs           []map[chan MeshMessage]struct{}
	coordMu             []sync.RWMutex

	eventsGlobalMu   sync.RWMutex
	eventsBroadcast  map[string][]chan []byte
	eventsSubs       map[string][]map[chan []byte]struct{}
	eventsMu         map[string][]sync.RWMutex
}`
	newStruct := `type LocalTeammateMesh struct {
	db                  db.Provider
	broadcast           []chan Task
	persist             []chan Task
	mu                  []sync.RWMutex
	subs                []map[chan Task]struct{}
	coordBroadcast      []chan MeshMessage
	coordSubs           []map[chan MeshMessage]struct{}
	coordMu             []sync.RWMutex

	eventsMu        []sync.RWMutex
	eventsBroadcast []map[string]chan []byte
	eventsSubs      []map[string]map[chan []byte]struct{}
}`
	strContent = strings.Replace(strContent, oldStruct, newStruct, 1)

	oldInit := `	lm := &LocalTeammateMesh{
		db:                  provider,
		broadcast:           make([]chan Task, numShards),
		persist:             make([]chan Task, numShards),
		mu:                  make([]sync.RWMutex, numShards),
		subs:                make([]map[chan Task]struct{}, numShards),
		coordBroadcast:      make([]chan MeshMessage, numShards),
		coordSubs:           make([]map[chan MeshMessage]struct{}, numShards),
		coordMu:             make([]sync.RWMutex, numShards),
	}`

	newInit := `	lm := &LocalTeammateMesh{
		db:                  provider,
		broadcast:           make([]chan Task, numShards),
		persist:             make([]chan Task, numShards),
		mu:                  make([]sync.RWMutex, numShards),
		subs:                make([]map[chan Task]struct{}, numShards),
		coordBroadcast:      make([]chan MeshMessage, numShards),
		coordSubs:           make([]map[chan MeshMessage]struct{}, numShards),
		coordMu:             make([]sync.RWMutex, numShards),
		eventsMu:            make([]sync.RWMutex, numShards),
		eventsBroadcast:     make([]map[string]chan []byte, numShards),
		eventsSubs:          make([]map[string]map[chan []byte]struct{}, numShards),
	}`

	strContent = strings.Replace(strContent, oldInit, newInit, 1)

	oldInitLoop := `		lm.coordBroadcast[i] = make(chan MeshMessage, 10000)
		lm.coordSubs[i] = make(map[chan MeshMessage]struct{})`

	newInitLoop := `		lm.coordBroadcast[i] = make(chan MeshMessage, 10000)
		lm.coordSubs[i] = make(map[chan MeshMessage]struct{})
		lm.eventsBroadcast[i] = make(map[string]chan []byte)
		lm.eventsSubs[i] = make(map[string]map[chan []byte]struct{})`

	strContent = strings.Replace(strContent, oldInitLoop, newInitLoop, 1)


	oldBroadcastMeshEvent := `func (lm *LocalTeammateMesh) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
	lm.eventsGlobalMu.RLock()
	if lm.eventsBroadcast == nil {
		lm.eventsGlobalMu.RUnlock()
		lm.eventsGlobalMu.Lock()
		if lm.eventsBroadcast == nil {
			lm.eventsBroadcast = make(map[string][]chan []byte)
			lm.eventsMu = make(map[string][]sync.RWMutex)
			lm.eventsSubs = make(map[string][]map[chan []byte]struct{})
		}
		lm.eventsGlobalMu.Unlock()
		lm.eventsGlobalMu.RLock()
	}

	if _, ok := lm.eventsBroadcast[topic]; !ok {
		lm.eventsGlobalMu.RUnlock()
		lm.eventsGlobalMu.Lock()
		if _, ok := lm.eventsBroadcast[topic]; !ok {
			lm.eventsBroadcast[topic] = make([]chan []byte, numShards)
			lm.eventsMu[topic] = make([]sync.RWMutex, numShards)
			lm.eventsSubs[topic] = make([]map[chan []byte]struct{}, numShards)
			for i := 0; i < numShards; i++ {
				lm.eventsBroadcast[topic][i] = make(chan []byte, 10000)
				lm.eventsSubs[topic][i] = make(map[chan []byte]struct{})
				go lm.runEvents(topic, i)
			}
		}
		lm.eventsGlobalMu.Unlock()
		lm.eventsGlobalMu.RLock()
	}

	broadcastArray := lm.eventsBroadcast[topic]
	lm.eventsGlobalMu.RUnlock()

	shardIdx := lm.getShard(topic)
	err := meshWithRetry(ctx, 3, func() error {
		select {
		case broadcastArray[shardIdx] <- payload:
			return nil
		default:
			return fmt.Errorf("LocalTeammateMesh events broadcast channel full")
		}
	})

	if err != nil {
		slog.Warn("LocalTeammateMesh events broadcast channel full, dropping message")
	}

	return nil
}`

	newBroadcastMeshEvent := `func (lm *LocalTeammateMesh) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
	shardIdx := lm.getShard(topic)

	lm.eventsMu[shardIdx].RLock()
	broadcastChan, exists := lm.eventsBroadcast[shardIdx][topic]
	lm.eventsMu[shardIdx].RUnlock()

	if !exists {
		lm.eventsMu[shardIdx].Lock()
		broadcastChan, exists = lm.eventsBroadcast[shardIdx][topic]
		if !exists {
			broadcastChan = make(chan []byte, 1000)
			lm.eventsBroadcast[shardIdx][topic] = broadcastChan
			lm.eventsSubs[shardIdx][topic] = make(map[chan []byte]struct{})
			go lm.runEvents(topic, shardIdx)
		}
		lm.eventsMu[shardIdx].Unlock()
	}

	err := meshWithRetry(ctx, 3, func() error {
		select {
		case broadcastChan <- payload:
			return nil
		default:
			return fmt.Errorf("LocalTeammateMesh events broadcast channel full")
		}
	})

	if err != nil {
		slog.Warn("LocalTeammateMesh events broadcast channel full, dropping message")
	}

	return nil
}`

	strContent = strings.Replace(strContent, oldBroadcastMeshEvent, newBroadcastMeshEvent, 1)

	oldSubscribeMeshEvents := `func (lm *LocalTeammateMesh) SubscribeMeshEvents(ctx context.Context, topic string) (<-chan []byte, error) {
	lm.eventsGlobalMu.RLock()
	if lm.eventsBroadcast == nil {
		lm.eventsGlobalMu.RUnlock()
		lm.eventsGlobalMu.Lock()
		if lm.eventsBroadcast == nil {
			lm.eventsBroadcast = make(map[string][]chan []byte)
			lm.eventsMu = make(map[string][]sync.RWMutex)
			lm.eventsSubs = make(map[string][]map[chan []byte]struct{})
		}
		lm.eventsGlobalMu.Unlock()
		lm.eventsGlobalMu.RLock()
	}

	if _, ok := lm.eventsBroadcast[topic]; !ok {
		lm.eventsGlobalMu.RUnlock()
		lm.eventsGlobalMu.Lock()
		if _, ok := lm.eventsBroadcast[topic]; !ok {
			lm.eventsBroadcast[topic] = make([]chan []byte, numShards)
			lm.eventsMu[topic] = make([]sync.RWMutex, numShards)
			lm.eventsSubs[topic] = make([]map[chan []byte]struct{}, numShards)
			for i := 0; i < numShards; i++ {
				lm.eventsBroadcast[topic][i] = make(chan []byte, 10000)
				lm.eventsSubs[topic][i] = make(map[chan []byte]struct{})
				go lm.runEvents(topic, i)
			}
		}
		lm.eventsGlobalMu.Unlock()
		lm.eventsGlobalMu.RLock()
	}

	muArray := lm.eventsMu[topic]
	subsArray := lm.eventsSubs[topic]
	lm.eventsGlobalMu.RUnlock()

	ch := make(chan []byte, 100)

	for i := 0; i < numShards; i++ {
		muArray[i].Lock()
		subsArray[i][ch] = struct{}{}
		muArray[i].Unlock()
	}

	go func() {
		<-ctx.Done()
		lm.eventsGlobalMu.RLock()
		if muArray, ok := lm.eventsMu[topic]; ok {
			subsArray := lm.eventsSubs[topic]
			for i := 0; i < numShards; i++ {
				muArray[i].Lock()
				delete(subsArray[i], ch)
				muArray[i].Unlock()
			}
		}
		lm.eventsGlobalMu.RUnlock()
		close(ch)
	}()

	return ch, nil
}

func (lm *LocalTeammateMesh) runEvents(topic string, shardIdx int) {
	lm.eventsGlobalMu.RLock()
	broadcastChan := lm.eventsBroadcast[topic][shardIdx]
	lm.eventsGlobalMu.RUnlock()

	for msg := range broadcastChan {
		lm.eventsGlobalMu.RLock()
		if muArray, ok := lm.eventsMu[topic]; ok {
			subsArray := lm.eventsSubs[topic]
			muArray[shardIdx].RLock()
			for ch := range subsArray[shardIdx] {
				select {
				case ch <- msg:
				default:
				}
			}
			muArray[shardIdx].RUnlock()
		}
		lm.eventsGlobalMu.RUnlock()
	}
}
`

	newSubscribeMeshEvents := `func (lm *LocalTeammateMesh) SubscribeMeshEvents(ctx context.Context, topic string) (<-chan []byte, error) {
	shardIdx := lm.getShard(topic)

	lm.eventsMu[shardIdx].RLock()
	_, exists := lm.eventsBroadcast[shardIdx][topic]
	lm.eventsMu[shardIdx].RUnlock()

	if !exists {
		lm.eventsMu[shardIdx].Lock()
		_, exists = lm.eventsBroadcast[shardIdx][topic]
		if !exists {
			lm.eventsBroadcast[shardIdx][topic] = make(chan []byte, 1000)
			lm.eventsSubs[shardIdx][topic] = make(map[chan []byte]struct{})
			go lm.runEvents(topic, shardIdx)
		}
		lm.eventsMu[shardIdx].Unlock()
	}

	ch := make(chan []byte, 100)

	lm.eventsMu[shardIdx].Lock()
	lm.eventsSubs[shardIdx][topic][ch] = struct{}{}
	lm.eventsMu[shardIdx].Unlock()

	go func() {
		<-ctx.Done()
		lm.eventsMu[shardIdx].Lock()
		if subs, ok := lm.eventsSubs[shardIdx][topic]; ok {
			delete(subs, ch)
		}
		lm.eventsMu[shardIdx].Unlock()
		close(ch)
	}()

	return ch, nil
}

func (lm *LocalTeammateMesh) runEvents(topic string, shardIdx int) {
	lm.eventsMu[shardIdx].RLock()
	broadcastChan := lm.eventsBroadcast[shardIdx][topic]
	lm.eventsMu[shardIdx].RUnlock()

	for msg := range broadcastChan {
		lm.eventsMu[shardIdx].RLock()
		subs := lm.eventsSubs[shardIdx][topic]
		for ch := range subs {
			select {
			case ch <- msg:
			default:
			}
		}
		lm.eventsMu[shardIdx].RUnlock()
	}
}
`
	strContent = strings.Replace(strContent, oldSubscribeMeshEvents, newSubscribeMeshEvents, 1)

	err = ioutil.WriteFile(filePath, []byte(strContent), 0644)
	if err != nil {
		fmt.Println("Error writing file:", err)
		return
	}
	fmt.Println("Successfully patched mesh.go")
}
