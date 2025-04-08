extends Panel
@onready var item_visual: Sprite2D = $CenterContainer/Panel/item
@onready var amount_text: Label = $CenterContainer/Panel/Label

signal out

func _ready() -> void:
	out.emit()

func update(slot: InvSlot):
	if !slot.item:
		item_visual.visible= false
		amount_text.visible = false
	else:
		item_visual.visible = true
		item_visual.texture = slot.item.icon
		if slot.amount > 1:
			amount_text.visible = true
			amount_text.text = str(slot.amount)

func _on_button_pressed() -> void:
	out.emit()
	pass # Replace with function body.
