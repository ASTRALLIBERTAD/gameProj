extends Control

var WorldName: String
@onready var t = get_tree()
@onready var y = preload("uid://d2oibegpqmv2b").instantiate()
@onready var i = get_node("/root/main/Terrain/Terrain1") as Terrain1

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
	if !get_tree() == null:
		if GameSeed.is_valid_int():
			SaveManager.WorldSeed = GameSeed
			var t = load("res://World.tscn").instantiate()
			get_tree().root.add_child(t)
			var i = get_node("/root/main/Terrain/Terrain1") as Terrain1
			i.seed_seed(SaveManager.WorldSeed)
			queue_free()
		elif GameSeed == "":
			var lp = RandomNumberGenerator.new()
			var t = hash(lp)
			SaveManager.WorldSeed = clamp(t, -2147483648, 2147483647)
			var y = load("res://World.tscn").instantiate()
			t.root.add_child(y)
			i.seed_seed(SaveManager.WorldSeed)
			queue_free()
		else:
			var t = hash(GameSeed)
			SaveManager.WorldSeed = clamp(t, -2147483648, 2147483647)
			var y = load("res://World.tscn").instantiate()
			get_tree().root.add_child(y)
			var i = get_node("/root/main/Terrain/Terrain1") as Terrain1
			i.seed_seed(SaveManager.WorldSeed)
			queue_free()
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
