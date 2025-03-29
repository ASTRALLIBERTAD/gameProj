extends Control


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	OS.request_permissions() 
	pass # Replace with function body.

func _on_play_pressed() -> void:
	get_tree().change_scene_to_file("res://SaveAndLoad/LoadMenu.scn")


func _on_settings_pressed() -> void:
	get_tree().change_scene_to_file("res://UserInterface/SettingMenu.scn")


func _on_exit_pressed() -> void:
	get_tree().quit()
