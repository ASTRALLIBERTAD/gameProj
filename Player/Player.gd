extends CharacterBody2D


@export var speed = 100

func _ready() -> void:
	SaveManager.player_node = self 
	emit_signal("player_ready")

func get_input():
	var input_direction = Input.get_vector("left", "right", "up", "down")
	velocity = input_direction * speed

func _physics_process(delta):
	get_input()
	move_and_slide()


func update_position(new_position: Vector2): 
	position = new_position
