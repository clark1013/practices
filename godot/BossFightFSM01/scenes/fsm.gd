extends Node2D

var cur_state: State
var prev_state: State

func _ready():
	# Enter Idle state first
	cur_state = get_child(0) as State
	prev_state = cur_state
	cur_state.enter()

func change_state(state):
	if prev_state.name == state:
		return
	
	cur_state = find_child(state) as State
	cur_state.enter()
	
	prev_state.exit()
	prev_state = cur_state
