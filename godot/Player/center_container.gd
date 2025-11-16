extends CenterContainer

@onready var inv: Inventory = preload("res://Collectibles/items/inventory.res")
@onready var slots: Array = get_children()

var index: int
# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	inv.update.connect(updated)
	
	pass # Replace with function body.

func updated():
	var inventory_slot : InvSlot = inv.slots[index]
	slots[0].update_to_slot(inventory_slot)
	print("happpe")

func selected_item(item_index):
	index = item_index
	var inventory_slot : InvSlot = inv.slots[index]
	slots[0].update_to_slot(inventory_slot)
	print("happpe")
	
