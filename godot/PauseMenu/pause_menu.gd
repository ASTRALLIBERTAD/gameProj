extends Control


# Called when the node enters the scene tree for the firs$TabContainert time.
func _ready() -> void:
	
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_save_game_pressed() -> void:
	print("kokok")
	
	$CanvasLayer/Coordinates.text = "PRINCE"
	pass # Replace with function body.