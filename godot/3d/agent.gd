extends Agent

@export var camera: Array[Camera3D] = []
const SPEED = 5.0
const JUMP_VELOCITY = 4.5
var current_index = 0
func _ready() -> void:
	init()
	update_cam()

func _physics_process(delta: float) -> void:
	update_movement($Camera3D/RayCast3D, $"../CharacterBody3D", delta)
	move_and_slide()

func update_cam():
	for i in range(camera.size()):
		camera[i].current = (i == current_index)

func _on_button_pressed() -> void:
	current_index =(current_index + 1)% camera.size()
	update_cam()
	pass # Replace with function body.
