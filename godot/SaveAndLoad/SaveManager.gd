extends Node

var base_path = "user://"
var LoadGame : String
var player_node: Rustplayer

func world_exist(world_name: String) -> bool:
	var world_file = base_path + world_name + "/" + world_name +".dat"
	return FileAccess.file_exists(world_file)
	


func save_game(name):
	var p = SaveManagerRust.new()
	p.save_game_rust(name)
	
	
	var SaveGameInfo = {
		"name" : name,
		"imgPath" : base_path + "games/" + name + "/" + name + ".png",
		"dateTime" : Time.get_unix_time_from_system()
	}
	var SaveGameJson = JSON.stringify(SaveGameInfo)
	
	var SaveGameFile = FileAccess.open(base_path + "games/" + name + "/" + name + "_saveGame.json", FileAccess.WRITE)
	SaveGameFile.store_string(SaveGameJson)
	
	var screenshot = get_viewport().get_texture().get_image()
	screenshot.save_png(base_path + "games/" + name + "/" + name + ".png")
	
	p.save_player_pos(name, player_node)

func optimize_autosave(name):
	var k = SaveManagerRust.new()
	k.save_game_rust(name)
	k.save_player_pos(name, player_node)
	print("Game saved successfully.")
	pass

func load_game(name):
	LoadGame = name	
	var i = SaveManagerRust.new()
	i.load_player_pos(name, player_node)


func delete_save(name):
	var dir_path = base_path + "games/" + name
	var dir = DirAccess.open(dir_path)
	if dir.dir_exists(dir_path):
		var files = dir.get_files()
		for file in files:
			dir.remove(dir_path + "/" + file)
		dir.remove(dir_path)
		print("Save game '" + name + "' deleted successfully.")
	else:
		print("Save game '" + name + "' not found.")

func get_world_name():
	return LoadGame
	
func auto_save():
	var world_name = get_world_name()
	print(world_name)
	if world_name != "":
		optimize_autosave(world_name)
	else :
		print("no world") 
