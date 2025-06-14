extends Node2dRust

@onready var scene = get_tree()
var peer = ENetMultiplayerPeer.new()

func _ready() -> void:
	$AutoSave.start()

@rpc("any_peer","call_local")
func add_player(pid):
	var plyr = preload("res://Player/multiplayers.scn").instantiate()
	plyr.name = str(pid)
	add_child(plyr)

func _on_auto_save_timeout() -> void:
	RustSaveManager1.auto_save()
	pass 

func _on_loading_pressed() -> void:
	print(OS.get_user_data_dir())
	var yt = RustSaveManager1.get_os()
	$po.text = yt
	$osl.text = OS.get_name()
	get_tree().paused = false
	pass 

func _on_saving_time_timeout() -> void:
	get_tree().paused = false
	RustSaveManager1.set_player_health(0)
	RustSaveManager1.rust_screenshot()
	scene.change_scene_to_file("res://SaveAndLoad/LoadMenu.scn")
	queue_redraw()
	queue_free()

func _on_menu_pressed() -> void:
	%TouchControls.visible = false
	get_tree().paused = true
	%Panel.visible = true
	pass # Replace with function body.

func _on_save_pressed() -> void:
	%TouchControls.visible = false
	%Panel.visible = false
	%CanvasLayer.visible = false
	%SavingTime.start()
	pass # Replace with function body.

func _on_back_pressed() -> void:
	%TouchControls.visible = true
	%Panel.visible = false
	get_tree().paused = false
	pass # Replace with function body.

func _on_add_player_pressed() -> void:
	peer.create_client("localhost", 55555)
	multiplayer.multiplayer_peer = peer
	pass # Replace with function body.

func _on_host_pressed() -> void:
	peer.create_server(5555, 3)
	multiplayer.multiplayer_peer = peer
	multiplayer.peer_connected.connect(
	func(pid):
		print(pid)
		rpc("add_player", pid)
		multiplayer.get_unique_id()
		)
	multiplayer.peer_disconnected.connect(
		func(pid):
			print(pid)
			get_node(str(pid)).queue_free()
	)
	%World.broadcast()
	$Broadcaster.start()
	RoomInfo.name = RustSaveManager1.load_game
	
	pass # Replace with function body.
var udp : PacketPeerUDP
var listner: PacketPeerUDP
@export var broadcastPort: int = 8912

var RoomInfo = {"name":"name", "playerCount": 0}
func _on_broadcaster_timeout() -> void:
	var data = JSON.stringify(RoomInfo)
	var packet = data.to_ascii_buffer()
	%World.broadcaster_timeout(packet)
	pass # Replace with function body.

func cleanUp():
	$Broadcaster.stop()
	if udp != null:
		udp.close()
