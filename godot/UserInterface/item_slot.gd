extends Control
@onready var inv: Inventory = preload("res://Collectibles/items/inventory.res")
@onready var slots: Array = $NinePatchRect/GridContainer.get_children()

var tapped_slot :Array= []
var first_slot : int = -1

func _ready() -> void:
	inv.update.connect(update_slots)
	update_slots()
	for b in range(slots.size()):
		var btn = slots[b].get_node("CenterContainer/Panel/Button") as Button
		btn.connect("pressed", func() -> void: _on_slot_pressed(b))

func _on_slot_pressed(index: int):
	print("slot pressed :" , index)
	if not tapped_slot.has(index):
		tapped_slot.append(index)
	if first_slot == -1:
		first_slot = index
		return
	
	if !inv.slots[first_slot].item.name:
		first_slot = -1
		tapped_slot.clear()
	
	if tapped_slot.size() == 2:
		po(first_slot, index)
		
		tapped_slot.clear()
		first_slot = -1

func  po(index1:int, index2:int):
	var t = inv.slots[0].item as Collectibles
	
	inv.insert(t, index1, index2)

func update_slots():
	for i in range(min(inv.slots.size(), slots.size())):
		slots[i].update(inv.slots[i])
		print("happpe")
