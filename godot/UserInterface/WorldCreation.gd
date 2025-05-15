extends Control

var WorldName: String
@onready var  newgame: = RustSaveManager1

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
	
	var world = preload("res://World.scn").instantiate() #res://World.tscn
	if !get_tree() == null:
		if GameSeed.is_valid_int():
			SaveManager.WorldSeed = GameSeed
			get_tree().root.add_child(world)
			var uop = world.get_node("/root/main/Terrain/Terrain1") as Terrain1
			uop.seed_seed(SaveManager.WorldSeed)
			queue_free()
		elif GameSeed == "":
			var lp: = RandomNumberGenerator.new()
			var ti = hash(lp)
			var yoj = clampi(ti, -2147483648, 2147483647)
			SaveManager.WorldSeed = yoj
			print(yoj)
			get_tree().root.add_child(world)
			var up = world.get_node("/root/main/Terrain/Terrain1") as Terrain1
			up.seed_seed(yoj)
			queue_free()
		else:
			var t: = hash(GameSeed)
			SaveManager.WorldSeed = clampi(t, -2147483648, 2147483647)
			get_tree().root.add_child(world)
			var upl = world.get_node("/root/main/Terrain/Terrain1") as Terrain1
			upl.seed_seed(SaveManager.WorldSeed)
			queue_free()
		RustSaveManager1.save_game_rust(WorldName)
		SaveManager.save_world()
	else:
		print("failed to  save a new game")
	
	newgame.save_game_rust(WorldName)

func _on_backbutton_pressed() -> void:
	get_tree().change_scene_to_file("res://SaveAndLoad/LoadMenu.scn")
	pass # Replace with function body.


func _on_back_pressed() -> void:
	get_tree().change_scene_to_file("res://SaveAndLoad/LoadMenu.scn")
	pass # Replace with function body.
