extends Control
@onready var inv: Inventory = preload("res://Collectibles/items/inventory.res")
@onready var slots: Array = $NinePatchRect/GridContainer.get_children()

var slot_num : Vector2i
var item : Dictionary
var item_count = 0

func _ready() -> void:
	inv.update.connect(update_slots)
	update_slots()
	for b in range(slots.size()):
		var btn = slots[b].get_node("CenterContainer/Panel/Button") as Button
		btn.connect("pressed", func() -> void: _on_slot_pressed(b))


func _on_slot_pressed(index: int):
	print("slot pressed :" , index)
	pass

func update_slots():
	for i in range(min(inv.slots.size(), slots.size())):
		slots[i].update(inv.slots[i])
		print("happpe")
