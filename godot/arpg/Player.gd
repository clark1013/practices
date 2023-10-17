extends CharacterBody2D

const SPEED = 100
const ACCELERATE = 100
const FRACTION = 1

func _physics_process(delta):
	var velocity_vector = Vector2.ZERO
	velocity_vector.x = Input.get_action_strength("ui_right") - Input.get_action_strength("ui_left")
	velocity_vector.y = Input.get_action_strength("ui_down") - Input.get_action_strength("ui_up")

	if velocity_vector != Vector2.ZERO:
		velocity = velocity_vector.normalized()
	else:
		velocity = velocity.move_toward(Vector2.ZERO, FRACTION)

	print(velocity)
	
	move_and_collide(velocity * delta)
