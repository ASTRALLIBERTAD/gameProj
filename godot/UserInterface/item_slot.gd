extends Control
@onready var inv: Inventory = preload("res://Collectibles/items/inventory.res")
@onready var slots: Array = $NinePatchRect/GridContainer.get_children()

var tapped_slot :Array= []
var first_slot : int = -1

@onready var tex: Sprite2D = $NinePatchRect/Sprite2D
var held_item: Texture = null

func _ready() -> void:
	inv.update.connect(update_slots)
	update_slots()
	for b in range(slots.size()):
		var btn = slots[b].get_node("CenterContainer/Panel/Button") as Button
		btn.connect("pressed", func() -> void: _on_slot_pressed(b))

func _on_slot_pressed(index: int) -> void:
	print("Slot pressed:", index)
	
	if not tapped_slot.has(index):
		tapped_slot.append(index)
		var item = inv.slots[index].item
		if item:
			held_item = item.icon
			tex.texture = held_item
			tex.visible = true
	else:
		# Unselecting the slot
		first_slot = -1
		tapped_slot.clear()
		held_item = null
		tex.visible = false
		return
	
	if first_slot == -1:
		if inv.slots[index].item.name:
			first_slot = index
			return
	
	# Cancel the operation if the first slot has no item or name is empty
	var first_item = inv.slots[first_slot].item
	if first_item == null or first_item.name == "" or first_item.name == null:
		print("First slot has no valid item name. Cancelling.")
		first_slot = -1
		tapped_slot.clear()
		held_item = null
		tex.visible = false
		return 
	
	if tapped_slot.size() == 2:
		po(first_slot, index)
		
		tapped_slot.clear()
		first_slot = -1
		held_item = null
		tex.visible = false


func _physics_process(delta: float) -> void:
	if held_item:
		tex.global_position = get_viewport().get_mouse_position()
		tex.visible = true

func  po(index1:int, index2:int):
	var t = inv.slots[0].item as Collectibles
	inv.insert(t, index1, index2)

func update_slots():
	for i in range(min(inv.slots.size(), slots.size())):
		slots[i].update(inv.slots[i])
		print("happpe")
