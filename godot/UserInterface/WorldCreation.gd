extends Control

var WorldName

func _on_playbuton_pressed() -> void:
	var WorldName = $BoxContainer/VBoxContainer/WorldNameInput.text
	print(WorldName)
	
	if WorldName == "":
		print("ERROR")
		return
	if SaveManager.world_exist(WorldName):
		print("world name already exist")
		return
	if !get_tree().change_scene_to_file("res://World.tscn") == null:
		SaveManager.LoadGame = WorldName
		var game = SaveManagerRust.new()
		
		game.save_game_rust(WorldName)
		
	else:
		print("failed to  save a new game")
	
	var game = SaveManagerRust.new()
	game.save_game_rust(WorldName)
		

func _on_backbutton_pressed() -> void:
	get_tree().change_scene_to_file("res://SaveAndLoad/LoadMenu.tscn")
	pass # Replace with function body.
