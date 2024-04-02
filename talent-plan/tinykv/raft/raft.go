// Copyright 2015 The etcd Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package raft

import (
	"errors"

	"math/rand"

	"github.com/pingcap-incubator/tinykv/log"
	pb "github.com/pingcap-incubator/tinykv/proto/pkg/eraftpb"
)

// None is a placeholder node ID used when there is no leader.
const None uint64 = 0

// StateType represents the role of a node in a cluster.
type StateType uint64

const (
	StateFollower StateType = iota
	StateCandidate
	StateLeader
)

var stmap = [...]string{
	"StateFollower",
	"StateCandidate",
	"StateLeader",
}

func (st StateType) String() string {
	return stmap[uint64(st)]
}

// ErrProposalDropped is returned when the proposal is ignored by some cases,
// so that the proposer can be notified and fail fast.
var ErrProposalDropped = errors.New("raft proposal dropped")

// Config contains the parameters to start a raft.
type Config struct {
	// ID is the identity of the local raft. ID cannot be 0.
	ID uint64

	// peers contains the IDs of all nodes (including self) in the raft cluster. It
	// should only be set when starting a new raft cluster. Restarting raft from
	// previous configuration will panic if peers is set. peer is private and only
	// used for testing right now.
	peers []uint64

	// ElectionTick is the number of Node.Tick invocations that must pass between
	// elections. That is, if a follower does not receive any message from the
	// leader of current term before ElectionTick has elapsed, it will become
	// candidate and start an election. ElectionTick must be greater than
	// HeartbeatTick. We suggest ElectionTick = 10 * HeartbeatTick to avoid
	// unnecessary leader switching.
	ElectionTick int
	// HeartbeatTick is the number of Node.Tick invocations that must pass between
	// heartbeats. That is, a leader sends heartbeat messages to maintain its
	// leadership every HeartbeatTick ticks.
	HeartbeatTick int

	// Storage is the storage for raft. raft generates entries and states to be
	// stored in storage. raft reads the persisted entries and states out of
	// Storage when it needs. raft reads out the previous state and configuration
	// out of storage when restarting.
	Storage Storage
	// Applied is the last applied index. It should only be set when restarting
	// raft. raft will not return entries to the application smaller or equal to
	// Applied. If Applied is unset when restarting, raft might return previous
	// applied entries. This is a very application dependent configuration.
	Applied uint64
}

func (c *Config) validate() error {
	if c.ID == None {
		return errors.New("cannot use none as id")
	}

	if c.HeartbeatTick <= 0 {
		return errors.New("heartbeat tick must be greater than 0")
	}

	if c.ElectionTick <= c.HeartbeatTick {
		return errors.New("election tick must be greater than heartbeat tick")
	}

	if c.Storage == nil {
		return errors.New("storage cannot be nil")
	}

	return nil
}

// Progress represents a follower’s progress in the view of the leader. Leader maintains
// progresses of all followers, and sends entries to the follower based on its progress.
type Progress struct {
	Match, Next uint64
}

type Raft struct {
	id uint64

	Term uint64
	Vote uint64

	// the log
	RaftLog *RaftLog

	// log replication progress of each peers
	Prs map[uint64]*Progress

	// this peer's role
	State StateType

	// votes records
	votes map[uint64]bool

	// msgs need to send
	msgs []pb.Message

	// the leader id
	Lead uint64

	// heartbeat interval, should send
	heartbeatTimeout int
	// baseline of election interval
	electionTimeout int
	// number of ticks since it reached last heartbeatTimeout.
	// only leader keeps heartbeatElapsed.
	heartbeatElapsed int
	// Ticks since it reached last electionTimeout when it is leader or candidate.
	// Number of ticks since it reached last electionTimeout or received a
	// valid message from current leader when it is a follower.
	electionElapsed int

	// leadTransferee is id of the leader transfer target when its value is not zero.
	// Follow the procedure defined in section 3.10 of Raft phd thesis.
	// (https://web.stanford.edu/~ouster/cgi-bin/papers/OngaroPhD.pdf)
	// (Used in 3A leader transfer)
	leadTransferee uint64

	// Only one conf change may be pending (in the log, but not yet
	// applied) at a time. This is enforced via PendingConfIndex, which
	// is set to a value >= the log index of the latest pending
	// configuration change (if any). Config changes are only allowed to
	// be proposed if the leader's applied index is greater than this
	// value.
	// (Used in 3A conf change)
	PendingConfIndex uint64
}

// newRaft return a raft peer with the given config
func newRaft(c *Config) *Raft {
	if err := c.validate(); err != nil {
		panic(err.Error())
	}
	// Your Code Here (2A).
	prs := make(map[uint64]*Progress)
	for _, p := range c.peers {
		prs[p] = &Progress{}
	}

	hs, cs, err := c.Storage.InitialState()
	if err != nil {
		panic(err.Error())
	}

	for _, n := range cs.Nodes {
		prs[n] = &Progress{}
	}

	log := newLog(c.Storage)
	log.committed = hs.Commit

	r := &Raft{
		id:               c.ID,
		Term:             hs.Term,
		Vote:             hs.Vote,
		RaftLog:          log,
		State:            StateFollower,
		heartbeatTimeout: c.HeartbeatTick,
		electionTimeout:  c.ElectionTick,
		votes:            make(map[uint64]bool),
		Prs:              prs,
	}
	r.becomeFollower(r.Term, None)
	return r
}

// sendAppend sends an append RPC with new entries (if any) and the
// current commit index to the given peer. Returns true if a message was sent.
func (r *Raft) sendAppend(to uint64) bool {
	// Your Code Here (2A).
	if to == r.id {
		return true
	}
	if r.State != StateLeader {
		return false
	}
	nextIdx := r.Prs[to].Next
	prevLogIndex := nextIdx - 1

	firstIndex := r.RaftLog.FirstIndex()
	if firstIndex > nextIdx {
		r.sendSnapshot(to)
		return true
	}

	prevLogTerm, _ := r.RaftLog.Term(prevLogIndex)
	entries := r.RaftLog.entriesAfterIndex(prevLogIndex)
	r.msgs = append(r.msgs, pb.Message{
		MsgType: pb.MessageType_MsgAppend,
		From:    r.id,
		To:      to,
		Term:    r.Term,
		LogTerm: prevLogTerm,
		Index:   prevLogIndex,
		Entries: entries,
		Commit:  r.RaftLog.committed,
	})
	return true
}

func (r *Raft) sendSnapshot(to uint64) {
	snapshot, err := r.RaftLog.storage.Snapshot()
	if err != nil {
		log.Errorf("get snapshot failed: %s", err)
		return
	}
	r.msgs = append(r.msgs, pb.Message{
		MsgType:  pb.MessageType_MsgSnapshot,
		From:     r.id,
		To:       to,
		Term:     r.Term,
		Snapshot: &snapshot,
	})
}

func (r *Raft) sendAppendResponse(to uint64, index uint64, reject bool) {
	r.msgs = append(r.msgs, pb.Message{
		MsgType: pb.MessageType_MsgAppendResponse,
		From:    r.id,
		To:      to,
		Term:    r.Term,
		Index:   index,
		Reject:  reject,
	})
}

// sendHeartbeat sends a heartbeat RPC to the given peer.
func (r *Raft) sendHeartbeat(to uint64) {
	// Your Code Here (2A).
	if to == r.id {
		return
	}
	r.msgs = append(r.msgs, pb.Message{
		MsgType: pb.MessageType_MsgHeartbeat,
		From:    r.id,
		To:      to,
		Term:    r.Term,
	})
}

func (r *Raft) sendHeartbeatResponse(to uint64, reject bool) {
	r.msgs = append(r.msgs, pb.Message{
		MsgType: pb.MessageType_MsgHeartbeatResponse,
		From:    r.id,
		To:      to,
		Term:    r.Term,
		Reject:  reject,
	})
}

func (r *Raft) sendRequestVote(to uint64) {
	if to == r.id {
		return
	}
	lastIndex := r.RaftLog.LastIndex()
	lastTerm, _ := r.RaftLog.Term(lastIndex)
	r.msgs = append(r.msgs, pb.Message{
		MsgType: pb.MessageType_MsgRequestVote,
		From:    r.id,
		To:      to,
		Term:    r.Term,
		LogTerm: lastTerm,
		Index:   lastIndex,
	})
}

func (r *Raft) sendRequestVoteResponse(to uint64, reject bool) {
	r.msgs = append(r.msgs, pb.Message{
		MsgType: pb.MessageType_MsgRequestVoteResponse,
		From:    r.id,
		To:      to,
		Term:    r.Term,
		Reject:  reject,
	})
}

// tick advances the internal logical clock by a single tick.
func (r *Raft) tick() {
	// Your Code Here (2A).
	switch r.State {
	case StateFollower, StateCandidate:
		r.electionElapsed += 1
		if r.electionElapsed >= r.electionTimeout+rand.Intn(r.electionTimeout*2) {
			r.Step(pb.Message{From: r.id, To: r.id, MsgType: pb.MessageType_MsgHup})
		}
	case StateLeader:
		r.heartbeatElapsed += 1
		if r.heartbeatElapsed >= r.heartbeatTimeout {
			for peer := range r.Prs {
				r.sendHeartbeat(peer)
			}
			r.heartbeatElapsed = 0
		}
	}
}

// becomeFollower transform this peer's state to Follower
func (r *Raft) becomeFollower(term uint64, lead uint64) {
	// Your Code Here (2A).
	r.State = StateFollower
	r.Term = term
	r.Lead = lead
	r.electionElapsed = 0
}

// becomeCandidate transform this peer's state to candidate
func (r *Raft) becomeCandidate() {
	// Your Code Here (2A).
	r.Term += 1
	r.State = StateCandidate
	r.votes = make(map[uint64]bool)
	r.votes[r.id] = true
	r.Vote = r.id
	r.electionElapsed = 0
}

// becomeLeader transform this peer's state to leader
func (r *Raft) becomeLeader() {
	// Your Code Here (2A).
	// NOTE: Leader should propose a noop entry on its term
	log.Infof("%d become leader in term %d", r.id, r.Term)
	r.State = StateLeader

	// Reset next and match indices for all peers
	lastIndex := r.RaftLog.LastIndex()
	for peerID := range r.Prs {
		r.Prs[peerID].Next = lastIndex + 1
		r.Prs[peerID].Match = lastIndex
	}

	noopEntry := pb.Entry{Term: r.Term, Index: lastIndex + 1, Data: nil}
	r.Step(pb.Message{
		MsgType: pb.MessageType_MsgPropose,
		Entries: []*pb.Entry{&noopEntry},
	})

	r.heartbeatElapsed = 0
}

// Step the entrance of handle message, see `MessageType`
// on `eraftpb.proto` for what msgs should be handled
func (r *Raft) Step(m pb.Message) error {
	// Your Code Here (2A).
	switch m.MsgType {
	case pb.MessageType_MsgHup:
		r.handleHup()
	case pb.MessageType_MsgBeat:
		r.handleBeat()
	case pb.MessageType_MsgPropose:
		r.handlePropose(m)
	case pb.MessageType_MsgRequestVote:
		r.handleRequestVote(m)
	case pb.MessageType_MsgRequestVoteResponse:
		r.handleRequestVoteResponse(m)
	case pb.MessageType_MsgHeartbeat:
		r.handleHeartbeat(m)
	case pb.MessageType_MsgHeartbeatResponse:
		r.handleHeartbeatResponse(m)
	case pb.MessageType_MsgAppend:
		r.handleAppendEntries(m)
	case pb.MessageType_MsgAppendResponse:
		r.handleAppendEntriesResponse(m)
	case pb.MessageType_MsgSnapshot:
		r.handleSnapshot(m)
	}

	return nil
}

func (r *Raft) handleHup() {
	if r.State == StateLeader {
		return
	}
	r.becomeCandidate()
	r.checkQuoram()
	for peer := range r.Prs {
		r.sendRequestVote(peer)
	}
}

func (r *Raft) handleBeat() {
	if r.State != StateLeader {
		return
	}
	for peer := range r.Prs {
		r.sendHeartbeat(peer)
	}
	r.heartbeatElapsed = 0
}

func (r *Raft) handlePropose(m pb.Message) {
	// fmt.Printf("handlePropose %+v\n", m)
	if r.State != StateLeader {
		return
	}
	LastIndex := r.RaftLog.LastIndex()
	r.Prs[r.id].Match = LastIndex + uint64(len(m.Entries))
	r.Prs[r.id].Next = r.Prs[r.id].Match + 1
	for i, entry := range m.Entries {
		entry.Term = r.Term
		entry.Index = LastIndex + uint64(i+1)
		r.RaftLog.entries = append(r.RaftLog.entries, *entry)
		if r.quoram() < 1 {
			r.RaftLog.committed = entry.Index
		}
	}

	for pr := range r.Prs {
		ok := r.sendAppend(pr)
		if !ok {
			log.Errorf("handle propose send append failed")
		}
	}

}

func (r *Raft) handleRequestVote(m pb.Message) {
	lastIndex := r.RaftLog.LastIndex()
	lastTerm, _ := r.RaftLog.Term(lastIndex)
	upToDate := true
	if m.LogTerm < lastTerm {
		upToDate = false
	} else if m.LogTerm == lastTerm && m.Index < lastIndex {
		upToDate = false
	}
	if upToDate {
		log.Infof("handleRequestVote %d->%d encounter up to date message, income term/index: %d/%d, my term/index: %d/%d", m.From, m.To, m.Term, m.Index, lastTerm, lastIndex)
	}

	switch r.State {
	case StateFollower:
		if m.Term > r.Term {
			r.Term = m.Term
			if upToDate {
				r.Vote = m.From
				r.sendRequestVoteResponse(m.From, false)
				return
			}
		} else if (r.Vote == m.From || r.Vote == 0) && upToDate {
			r.Vote = m.From
			r.sendRequestVoteResponse(m.From, false)
			return
		}
	case StateCandidate, StateLeader:
		if m.Term > r.Term {
			r.becomeFollower(m.Term, None)
			if upToDate {
				r.Vote = m.From
				r.sendRequestVoteResponse(m.From, false)
				return
			}
		}
	}

	r.sendRequestVoteResponse(m.From, true)
}

func (r *Raft) handleRequestVoteResponse(m pb.Message) {
	switch r.State {
	case StateFollower:
	case StateCandidate:
		if !m.Reject {
			r.votes[m.From] = true
		} else {
			r.votes[m.From] = false
		}
		r.checkQuoram()
	case StateLeader:
	}
}

func (r *Raft) checkQuoram() {
	approved := 0
	rejected := 0
	voters := make([]uint64, 0, 5)
	for voter, ok := range r.votes {
		if ok {
			approved++
			voters = append(voters, voter)
		} else {
			rejected++
		}
	}
	if approved > r.quoram() {
		r.becomeLeader()
		log.Infof("and voters are: %v", voters)
	}
	if rejected > r.quoram() {
		r.becomeFollower(r.Term, None)
	}
}

// handleAppendEntries handle AppendEntries RPC request
func (r *Raft) handleAppendEntries(m pb.Message) {
	// Your Code Here (2A).
	// fmt.Printf("handleAppendEntries %+v %+v \n", r.id, m.Commit)
	if m.Term >= r.Term {
		r.becomeFollower(m.Term, m.From)
	} else {
		r.sendAppendResponse(m.From, m.Index, true)
		return
	}

	t, err := r.RaftLog.Term(m.Index)
	if err != nil || t != m.LogTerm {
		r.sendAppendResponse(m.From, m.Index, true)
		return
	}

	for _, entry := range m.Entries {
		t, err := r.RaftLog.Term(entry.Index)
		if err != nil {
			r.RaftLog.entries = append(r.RaftLog.entries, *entry)
		} else if t != entry.Term {
			r.RaftLog.removeEntriesSinceIndex(entry.Index)
			r.RaftLog.entries = append(r.RaftLog.entries, *entry)
		} else {
			continue
		}
	}
	lastIndex := m.Index + uint64(len(m.Entries))
	if m.Commit > r.RaftLog.committed {
		if lastIndex < m.Commit {
			r.RaftLog.committed = lastIndex
		} else {
			r.RaftLog.committed = m.Commit
		}
	}
	r.sendAppendResponse(m.From, m.Index+uint64(len(m.Entries)), false)
}

func (r *Raft) handleAppendEntriesResponse(m pb.Message) {
	// fmt.Printf("handleAppendEntriesResponse %d %+v\n", r.id, m)
	if m.Reject && m.Term > r.Term {
		r.becomeFollower(m.Term, m.From)
	}
	switch r.State {
	case StateFollower, StateCandidate:
	case StateLeader:
		fromProgress := r.Prs[m.From]
		if m.Reject {
			fromProgress.Next -= 1
			fromProgress.Match -= 1
			r.sendAppend(m.From)
			return
		}

		fromProgress.Match = m.Index
		fromProgress.Next = m.Index + 1

		var replicas int
		for _, pr := range r.Prs {
			if pr.Match >= fromProgress.Match {
				replicas += 1
			}
		}

		if replicas > r.quoram() && fromProgress.Match > r.RaftLog.committed {
			t, _ := r.RaftLog.Term(fromProgress.Match)
			if t == r.Term {
				r.RaftLog.committed = fromProgress.Match
				for pr := range r.Prs {
					r.sendAppend(pr)
				}
			}
		}

		if fromProgress.Match < r.RaftLog.LastIndex() {
			r.sendAppend(m.From)
		}
	}
}

// handleHeartbeat handle Heartbeat RPC request
func (r *Raft) handleHeartbeat(m pb.Message) {
	// Your Code Here (2A).
	if m.Term >= r.Term {
		r.becomeFollower(m.Term, m.From)
	}
	r.sendHeartbeatResponse(m.From, false)
}

func (r *Raft) handleHeartbeatResponse(m pb.Message) {
	if m.Term > r.Term {
		r.becomeFollower(m.Term, m.From)
	}
	switch r.State {
	case StateFollower, StateCandidate:
	case StateLeader:
		fromProgress := r.Prs[m.From]
		lastIndex := r.RaftLog.LastIndex()
		if lastIndex > fromProgress.Match {
			r.sendAppend(m.From)
		}
	}

}

func (r *Raft) quoram() int {
	return len(r.Prs) / 2
}

// handleSnapshot handle Snapshot RPC request
func (r *Raft) handleSnapshot(m pb.Message) {
	// Your Code Here (2C).
	if r.Term < m.Term {
		r.Term = m.Term
		if r.State != StateFollower {
			r.becomeFollower(r.Term, None)
		}
	}
	if m.Term < r.Term {
		return
	}

	metaData := m.Snapshot.Metadata
	shotIndex := metaData.Index
	shotTerm := metaData.Term
	shotConf := metaData.ConfState

	if shotIndex < r.RaftLog.committed || shotIndex < r.RaftLog.FirstIndex() {
		return
	}
	if r.Lead != m.From {
		r.Lead = m.From
	}

	// 丢弃之前的所有 entry
	if len(r.RaftLog.entries) > 0 {
		if shotIndex >= r.RaftLog.LastIndex() {
			r.RaftLog.entries = nil
		} else {
			r.RaftLog.entries = r.RaftLog.entries[shotIndex-r.RaftLog.FirstIndex()+1:]
		}
	}

	r.RaftLog.committed = shotIndex
	r.RaftLog.applied = shotIndex
	r.RaftLog.stabled = shotIndex

	// 集群节点变更
	if shotConf != nil {
		r.Prs = make(map[uint64]*Progress)
		for _, node := range shotConf.Nodes {
			r.Prs[node] = &Progress{}
			r.Prs[node].Next = r.RaftLog.LastIndex() + 1
			r.Prs[node].Match = 0
		}
	}

	if r.RaftLog.LastIndex() < shotIndex {
		// 加一个空条目，以指明 lastIndex 和 lastTerm 与快照一致
		entry := pb.Entry{
			EntryType: pb.EntryType_EntryNormal,
			Index:     shotIndex,
			Term:      shotTerm,
		}
		r.RaftLog.entries = append(r.RaftLog.entries, entry)
	}

	r.RaftLog.pendingSnapshot = m.Snapshot
	r.sendAppendResponse(m.From, r.RaftLog.LastIndex(), false)
}

func (r *Raft) softState() *SoftState {
	return &SoftState{
		Lead:      r.Lead,
		RaftState: r.State,
	}
}

func (r *Raft) hardState() pb.HardState {
	return pb.HardState{
		Term:   r.Term,
		Vote:   r.Vote,
		Commit: r.RaftLog.committed,
	}
}

// addNode add a new node to raft group
func (r *Raft) addNode(id uint64) {
	// Your Code Here (3A).
}

// removeNode remove a node from raft group
func (r *Raft) removeNode(id uint64) {
	// Your Code Here (3A).
}
