extends Control

@export var LoadButton : PackedScene
var SaveToLoad

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var dir = DirAccess.get_directories_at("user://games")
	for i in dir:
		var button : Button = LoadButton.instantiate()
		button.LoadButtonDown.connect(OnLoadButtonDown)
		var file_path = "user://games/%s/%s_saveGame.json" % [i, i]
		
		var file = FileAccess.open( file_path, FileAccess.READ)
		var content = file.get_as_text()
		var obj = JSON.parse_string(content)
		button.SetupButton(obj)
		button.text = obj.name
		$Panel/ScrollContainer/LoadButtons.add_child(button)
	pass # Replace with function body.


func OnLoadButtonDown(date, saveName, imagePath):
	$Name.text = saveName
	$Date.text = date
	SaveToLoad = saveName
	$ScreenShot.texture = LoadImageTexture(imagePath)
	pass

func LoadImageTexture(path : String):
	var loadedImage = Image.new()
	var error = loadedImage.load(path)
	
	if error != OK:
		print("image failed to load")
		return
	return ImageTexture.create_from_image(loadedImage)


func _on_load_scene_button_down() -> void:
	var world_scene = load("res://World.tscn").instantiate()
	get_tree().root.add_child(world_scene)
	hide()
	SaveManager.load_game(SaveToLoad)
