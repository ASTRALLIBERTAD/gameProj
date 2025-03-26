extends Control
@onready var inv: Inventory = preload("res://Collectibles/items/inventory.res")
@onready var slots: Array = $NinePatchRect/GridContainer.get_children()

var slot_num : Vector2i
var item : Dictionary
var item_count = 0

func _ready() -> void:
	inv.update.connect(update_slots)
	update_slots()

func update_slots():
	for i in range(min(inv.slots.size(), slots.size())):
		slots[i].update(inv.slots[i])
		print("happpe")
