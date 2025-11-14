extends Panel
@onready var background: Sprite2D = $Sprite2D
@onready var item_visual: Sprite2D = $CenterContainer/Panel/item
@onready var amount_text: Label = $CenterContainer/Panel/Label

signal out

func _ready() -> void:
	out.emit()

func update_to_slot(slot: InvSlot):
	redraw(slot)
	if !slot.item.name:
		item_visual.visible= false
		amount_text.visible = false
		amount_text.text = str(0)
	else:
		item_visual.visible = true
		item_visual.texture = slot.item.icon
		if slot.item.amount > 1:
			amount_text.visible = true
			var t = slot.item.amount
			amount_text.text = str(t)

func redraw(slot: InvSlot):
	if slot.item.amount <= 1:
		amount_text.visible = false
		var t = slot.item.amount
		amount_text.text = str(t)

func _on_button_pressed() -> void:
	out.emit()
	pass # Replace with function body.
