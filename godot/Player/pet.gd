extends CharacterBody2D
@onready var player = %PLAYERS

const SPEED = 100.0
const JUMP_VELOCITY = -400.0
const MINIMUM_DISTANCE = 200

func _physics_process(delta: float) -> void:
	# Add the gravity.
	var distance = global_position.distance_to(player.global_position)
	if distance > 20:
		var direction = global_position.direction_to(player.global_position).normalized()
		velocity = direction * SPEED
		move_and_slide()
