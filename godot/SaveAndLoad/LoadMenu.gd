extends Control
@export var LoadButton : PackedScene
var SaveToLoad
var GameTerrain: int

func _ready() -> void:
	if get_tree().root.has_node("/root/main"):
		get_tree().root.get_node("/root/main").queue_free()
	var dir = DirAccess.get_directories_at( RustSaveManager1.get_os() + "/games")
	for i in dir:
		var button : Button = LoadButton.instantiate()
		button.LoadButtonDown.connect(OnLoadButtonDown)
		var file_path: String = RustSaveManager1.get_os() + "/games/%s/%s_saveGame.json" % [i, i]
		
		var file: = FileAccess.open( file_path, FileAccess.READ)
		var content: = file.get_as_text()
		var obj = JSON.parse_string(content)
		button.SetupButton(obj)
		button.text = obj.name
		

		$CanvasLayer/Panel/ScrollContainer/LoadButtons.add_child(button)
	queue_redraw()
	pass # Replace with function body.


func OnLoadButtonDown(date, saveName, imagePath, seedGame):
	$CanvasLayer/Name.text = saveName
	$CanvasLayer/Date.text = date
	$CanvasLayer/Seed.text = str(seedGame)
	SaveToLoad = saveName
	GameTerrain = seedGame
	$CanvasLayer/ScreenShot.texture = LoadImageTexture(imagePath)
	pass

func LoadImageTexture(path : String):
	var loadedImage: = Image.new()
	var error: = loadedImage.load(path)
	
	if error != OK:
		print("image failed to load")
		return
	return ImageTexture.create_from_image(loadedImage)
	
	


func _on_timer_timeout() -> void:
	var world_scene: = preload("res://World.scn").instantiate()
	get_tree().root.add_child(world_scene)
	queue_free()
	RustSaveManager1.load_game(SaveToLoad)
	var u: Terrain1 = get_node("/root/main/Terrain/Terrain1") as Terrain1
	u.seed_seed(GameTerrain)

func _on_delete_pressed() -> void:
	RustSaveManager1.delete_save(SaveToLoad)
	get_tree().reload_current_scene()
	queue_redraw()
	pass # Replace with function body.

func _on_new_pressed() -> void:
	get_tree().change_scene_to_file("res://UserInterface/WorldCreation.scn")
	pass # Replace with function body.

func _on_back_pressed() -> void:
	get_tree().change_scene_to_file("res://Main/MainMenu.scn")
	pass # Replace with function body.


func _on_multiplayer_pressed() -> void:
	get_tree().change_scene_to_file("res://world/multiplayer.scn")
	pass # Replace with function body.


func _on_load_scene_button_down() -> void:
	$Timer.start()
	pass # Replace with function body.
