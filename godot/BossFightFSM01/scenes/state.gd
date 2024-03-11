extends Node2D
class_name State

@onready var debug_label = owner.find_child("Debug")
@onready var animation_player = owner.find_child("AnimationPlayer")

func _ready():
	set_physics_process(false)

func enter():
	set_physics_process(true)
	
func exit():
	set_physics_process(false)

func transition():
	pass

func _physics_process(delta):
	transition()
	debug_label.text = name
