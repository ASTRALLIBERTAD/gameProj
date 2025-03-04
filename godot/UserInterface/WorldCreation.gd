extends Control

var WorldName: String
@onready var t: = get_tree()
@onready var y: = preload("uid://d2oibegpqmv2b").instantiate() #res://World.tscn
@onready var  newgame: = SaveManagerRust.new()

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
	var Terrain = get_node("/root/main/Terrain/Terrain1") as Terrain1
	var world = preload("uid://d2oibegpqmv2b").instantiate() #res://World.tscn
	if !get_tree() == null:
		if GameSeed.is_valid_int():
			SaveManager.WorldSeed = GameSeed
			get_tree().root.add_child(world)
			Terrain.seed_seed(SaveManager.WorldSeed)
			queue_free()
		elif GameSeed == null:
			var lp: = RandomNumberGenerator.new()
			var ti = hash(lp)
			var m = randi_range(-2147483648, 2147483647)
			SaveManager.WorldSeed = m
			print(m)
			get_tree().root.add_child(world)
			Terrain.seed_seed(m)
			queue_free()
		else:
			var t: = hash(GameSeed)
			SaveManager.WorldSeed = clamp(t, -2147483648, 2147483647)
			get_tree().root.add_child(world)
			var i: = get_node("/root/main/Terrain/Terrain1") as Terrain1
			i.seed_seed(SaveManager.WorldSeed)
			queue_free()
		var game: = SaveManagerRust.new()
		game.save_game_rust(WorldName)
		SaveManager.save_world(WorldName)
	else:
		print("failed to  save a new game")
	
	newgame.save_game_rust(WorldName)

func _on_backbutton_pressed() -> void:
	get_tree().change_scene_to_file("res://SaveAndLoad/LoadMenu.tscn")
	pass # Replace with function body.


func _on_back_pressed() -> void:
	get_tree().change_scene_to_file("res://SaveAndLoad/LoadMenu.tscn")
	pass # Replace with function body.
