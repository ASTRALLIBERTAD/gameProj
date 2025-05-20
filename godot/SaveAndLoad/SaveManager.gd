extends Node

var base_path: String
var LoadGame : String 
var player_node: Rustplayer
var WorldSeed: int


func world_exist(world_name: String) -> bool:
	var world_file: String = RustSaveManager1.get_os() + world_name + "/" + world_name +".dat"
	return FileAccess.file_exists(world_file)
	

# func save_game():
# 	var world_name = get_world_name()
# 	RustSaveManager1.save_player_pos(world_name)
	
# 	print(world_name)
# 	var screenshot: = get_viewport().get_texture().get_image()
# 	screenshot.save_png(RustSaveManager1.get_os() + "games/" + world_name + "/" + world_name + ".png")
	


func save_world():
	var world_name = get_world_name()
	print(world_name)
	
	var SaveGameInfo := {
		"name" : world_name,
		"imgPath" : RustSaveManager1.get_os() + "games/" + world_name + "/" + world_name + ".png",
		"dateTime" : Time.get_unix_time_from_system(),
		"seed": WorldSeed
	}
	var SaveGameJson := JSON.stringify(SaveGameInfo)
	
	var SaveGameFile := FileAccess.open( RustSaveManager1.get_os() + "games/" + world_name + "/" + world_name + "_saveGame.json", FileAccess.WRITE)
	SaveGameFile.store_string(SaveGameJson)
	
	var screenshot := get_viewport().get_texture().get_image()
	screenshot.save_png(RustSaveManager1.get_os() + "games/" + world_name + "/" + world_name + ".png")
	


# func save(name: String):
# 	RustSaveManager1.save_game_rust(name)
# 	pass

# func delete_save(name):
# 	var dir_path: String = RustSaveManager1.get_os() + "games/" + name
# 	var dir := DirAccess.open(dir_path)
# 	if dir.dir_exists(dir_path):
# 		var files := dir.get_files()
# 		for file in files:
# 			dir.remove(dir_path + "/" + file)
# 		dir.remove(dir_path)
# 		print("Save game '" + name + "' deleted successfully.")
# 	else:
# 		print("Save game '" + name + "' not found.")

func get_world_name():
	return LoadGame
