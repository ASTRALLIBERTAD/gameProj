extends Control

var WorldName

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_playbuton_pressed() -> void:
	var WorldName = $BoxContainer/VBoxContainer/WorldNameInput.text
	print(WorldName)
	if !get_tree().change_scene_to_file("res://World.tscn") == null:
		SaveManager.LoadGame = WorldName
		SaveManager.save_game(WorldName)
		
		
	else:
		print("failed to  save a new game")
	pass # Replace with function body.
