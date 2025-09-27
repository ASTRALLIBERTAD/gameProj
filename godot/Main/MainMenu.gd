extends Control

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	OS.request_permissions() 
	$Transition/ColorRect.visible = false
	pass # Replace with function body.

func _on_play_pressed() -> void:
	$Transition/ColorRect.visible = true
	$Transition.play("fade_out")
	#get_tree().change_scene_to_file("res://joke/joke.scn")


func _on_settings_pressed() -> void:
	get_tree().change_scene_to_file("res://UserInterface/SettingMenu.scn")


func _on_exit_pressed() -> void:
	get_tree().quit()


func _on_transition_animation_finished(anim_name: StringName) -> void:
	get_tree().change_scene_to_file("res://SaveAndLoad/LoadMenu.scn")
	pass # Replace with function body.
