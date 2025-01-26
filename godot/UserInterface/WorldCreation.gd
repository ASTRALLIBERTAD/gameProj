extends Control

var WorldName: String

func _on_playbuton_pressed() -> void:
	var WorldName = %WorldNameInput.text
	SaveManager.LoadGame = WorldName
	var GameSeed = %Seed.text.strip_edges()
	print(WorldName)
	
	if WorldName == "":
		print("ERROR")
		return
	if SaveManager.world_exist(WorldName):
		print("world name already exist")
		return
	if !get_tree().change_scene_to_file("res://World.tscn") == null:
		if GameSeed.is_valid_int():
			SaveManager.WorldSeed = GameSeed
		elif GameSeed == "":
			var lp = RandomNumberGenerator.new()
			var t = hash(lp)
			SaveManager.WorldSeed = t 
		else:
			var t = hash(GameSeed)
			SaveManager.WorldSeed = t
		var game = SaveManagerRust.new()
		
		game.save_game_rust(WorldName)
		SaveManager.save_world(WorldName)
		
	else:
		print("failed to  save a new game")
	
	
	var game = SaveManagerRust.new()
	game.save_game_rust(WorldName)

func _on_backbutton_pressed() -> void:
	get_tree().change_scene_to_file("res://SaveAndLoad/LoadMenu.tscn")
	pass # Replace with function body.
