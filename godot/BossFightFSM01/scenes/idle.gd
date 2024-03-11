extends State

@onready var collision = %CollisionShape2D
@onready var progress_bar = %ProgressBar

var player_entered: bool = false:
	set(value):
		player_entered = value
		collision.set_deferred("disabled", value)
		progress_bar.set_deferred("visible", value)

func _on_player_detection_body_entered(body):
	player_entered = true

func transition():
	if player_entered:
		get_parent().change_state("Summon")
