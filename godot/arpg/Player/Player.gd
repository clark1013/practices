extends CharacterBody2D

const SPEED = 100
const ACCELERATE = 10
const FRACTION = 10

var animationTree
var animationState

func _ready():
	animationTree = $AnimationTree
	animationState = animationTree.get("parameters/playback")


func _physics_process(delta):
	var input_vector = Vector2.ZERO
	input_vector.x = Input.get_action_strength("ui_right") - Input.get_action_strength("ui_left")
	input_vector.y = Input.get_action_strength("ui_down") - Input.get_action_strength("ui_up")
	input_vector = input_vector.normalized()

	if input_vector != Vector2.ZERO:
		animationTree.set("parameters/Idle/blend_position", input_vector)
		animationTree.set("parameters/Run/blend_position", input_vector)
		animationState.travel("Run")
		velocity = input_vector * SPEED
	else:
		animationState.travel("Idle")
		velocity = velocity.move_toward(Vector2.ZERO, FRACTION)

	move_and_slide()
