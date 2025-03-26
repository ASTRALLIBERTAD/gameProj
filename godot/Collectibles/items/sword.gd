extends StaticBody2D

@export var item: Collectibles
var player = null

func _on_area_2d_body_entered(body: Node2D) -> void:
	if body.has_method("player"):
		player = body
		player_collect()
		await get_tree().create_timer(0.1).timeout
		self.queue_free()
	pass # Replace with function body.

func player_collect():
	player.collect_items(item)
