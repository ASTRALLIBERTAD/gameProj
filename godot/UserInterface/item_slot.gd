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
	var t = inv.slots[index].item.name
	print("iii", t)
	po(0,1)

func  po(from:int,into:int):
	var t = inv.slots[0].item as Collectibles
	
	inv.insert(t, 0)
	#var l = inv.slots[from].item 
	#inv.slots[from].item = inv.slots[into].item
	#inv.slots[into].item = l
	#var u = inv.slots[from].amount
	#inv.slots[from].amount = inv.slots[into].amount
	#inv.slots[into].amount = u

func update_slots():
	for i in range(min(inv.slots.size(), slots.size())):
		slots[i].update(inv.slots[i])
		print("happpe")
