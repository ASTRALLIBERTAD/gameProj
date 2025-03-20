extends Panel
@onready var item_visual: Sprite2D = $item

func update(item: Collectibles):
	if !item:
		item_visual.visible= false
	else:
		item_visual.visible = true
		item_visual.texture = item.icon
	pass
