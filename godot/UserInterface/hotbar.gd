extends HBoxContainer
@onready var inv: Inventory = preload("res://Collectibles/items/inventory.res")
@onready var slots: Array = get_children()

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	inv.update.connect(updated)
	updated()
	
	pass # Replace with function body.


func updated():
	for i in range(slots.size()):
		var inventory_slot : InvSlot = inv.slots[i]
		slots[i].update_to_slot(inventory_slot)
		print("happpe")
