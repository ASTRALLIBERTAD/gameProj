extends Control

var WorldName: String
@onready var  newgame: = RustSaveManager1

func _on_timer_timeout() -> void:
	var WorldName = %WorldNameInput.text
	RustSaveManager1.load_game = WorldName
	var GameSeed = %Seed.text.strip_edges()
	print(WorldName)
	
	if WorldName == "":
		print("ERROR")
		return
	#if SaveManager.world_exist(WorldName):
		#print("world name already exist")
		#return
	
	var world = preload("res://World.scn").instantiate() #res://World.tscn
	if !get_tree() == null:
		if GameSeed.is_valid_int():
			RustSaveManager1.world_seed = int(GameSeed)
			get_tree().root.add_child(world)
			var uop = world.get_node("/root/main/Terrain/Terrain1") as Terrain1
			uop.seed_seed(RustSaveManager1.world_seed)
			queue_free()
		elif GameSeed == "":
			var lp: = RandomNumberGenerator.new()
			var ti = hash(lp)
			var yoj = clampi(ti, -2147483648, 2147483647)
			RustSaveManager1.world_seed = yoj
			print(yoj)
			get_tree().root.add_child(world)
			var up = world.get_node("/root/main/Terrain/Terrain1") as Terrain1
			up.seed_seed(yoj)
			queue_free()
		else:
			var t: = hash(GameSeed)
			RustSaveManager1.world_seed = clampi(t, -2147483648, 2147483647)
			get_tree().root.add_child(world)
			var upl = world.get_node("/root/main/Terrain/Terrain1") as Terrain1
			upl.seed_seed(RustSaveManager1.world_seed)
			queue_free()
		
		RustSaveManager1.save_game_rust(WorldName)
		RustSaveManager1.save_world()
	else:
		print("failed to  save a new game")
	
	newgame.save_game_rust(WorldName)

func _on_backbutton_pressed() -> void:
	get_tree().change_scene_to_file("res://SaveAndLoad/LoadMenu.scn")
	pass # Replace with function body.


func _on_back_pressed() -> void:
	get_tree().change_scene_to_file("res://SaveAndLoad/LoadMenu.scn")
	pass # Replace with function body.


func _on_playbuton_pressed() -> void:
	$Timer.start()
	RustSaveManager1.set_player_health(5)
	pass # Replace with function body.
