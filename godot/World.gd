extends Node2dRust

@onready var scene = get_tree()
var peer = ENetMultiplayerPeer.new()

func _ready() -> void:
	$AutoSaveTimer.start()
	GlobalNodeManager.register_terrain($"../Terrain/Terrain1")

@rpc("any_peer","call_local")
func add_player(pid):
	var plyr = preload("res://Player/players.scn").instantiate() as Rustplayer
	plyr.name = str(pid)
	add_child(plyr)
	
	plyr.set_multiplayer_authority(pid)

func _on_auto_save_timeout() -> void:
	RustSaveManager1.auto_save()
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


func _on_host_pressed() -> void:
	peer.create_server(5555, 3)
	multiplayer.multiplayer_peer = peer
	%World.broadcast()
	$Broadcaster.start()
	RoomInfo.name = RustSaveManager1.load_game
	var id = multiplayer.get_unique_id()
	$PLAYERS.set_multiplayer_authority(id)
	
	
	multiplayer.peer_connected.connect(
	func(pid):
		print(pid)
		var terrain = get_node("/root/main/Terrain/Terrain1") as Terrain1
		var seed = terrain.seedser
		$"..".rpc("seed", seed)
		
		rpc("add_player", pid)
		
		var i = multiplayer.get_unique_id()
		player_node_names.append(str(pid))
	
		)
	multiplayer.peer_disconnected.connect(
		func(pid):
			print(pid)
			get_node(str(pid)).queue_free()
			player_node_names.erase(str(pid))
	)
	
	pass # Replace with function body.
var udp : PacketPeerUDP
var listner: PacketPeerUDP
@export var broadcastPort: int = 8912

var RoomInfo = {"name":"name", "playerCount": 0}
func _on_broadcaster_timeout() -> void:
	var data = JSON.stringify(RoomInfo)
	var packet = data.to_ascii_buffer()
	%World.broadcaster_timeout(packet)
	print(packet)
	pass # Replace with function body.

func cleanUp():
	$Broadcaster.stop()
	if udp != null:
		udp.close()
