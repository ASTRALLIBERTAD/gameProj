extends Control


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_playbuton_pressed() -> void:
	var WorldName = $BoxContainer/VBoxContainer/WorldNameInput.text
	get_tree().change_scene_to_file("res://World.tscn")
	SaveManager.save_game(WorldName)
	print(WorldName)
	pass # Replace with function body.
