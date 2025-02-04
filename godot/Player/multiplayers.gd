extends MultiPlayerRust
@onready var u = get_node("/root/main/Terrain/Terrain1") as Terrain1

@rpc( "any_peer","unreliable", "call_local")
func trs(pid):
	if is_multiplayer_authority():
		u.tile(int(str(pid)))
	if !is_multiplayer_authority():
		if name == pid:
			u.tile(int(str(pid)))

func _on_tile_timeout() -> void:
	if is_multiplayer_authority():
		rpc("trs", name)
	else :
		rpc_id(1, "trs", name)
		print("kfk")
	print("fp")
