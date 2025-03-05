# extends Node2dRust

# @onready var scene = get_tree()
# var peer = ENetMultiplayerPeer.new()
# # var session: NakamaSession
# # var client: NakamaClient
# # var socket: NakamaSocket
# func _ready() -> void:
# 	$AutoSave.start()

# # func onMatchState(state : NakamaRTAPI.MatchData):
# # 	print("data is : " + str(state.data))

# # func onMatchPresence(presence : NakamaRTAPI.MatchPresenceEvent):
# # 	print(presence)

# func onSocketClosed():
# 	print("Socket Closed")

# func onSocketReceivedError(err):
# 	print("Socket Error:" + str(err))

# func conectedsocket():
# 	print("connected socket")

# func add_player(pid):
# 	var plyr = preload("res://Player/multiplayers.tscn").instantiate()
# 	plyr.name = str(pid)
# 	add_child(plyr)

# func _on_auto_save_timeout() -> void:
# 	SaveManager.auto_save()
# 	pass 

# func _on_loading_pressed() -> void:
# 	print(OS.get_user_data_dir())
# 	var yt = SaveManager.get_os()
# 	$po.text = yt
# 	$osl.text = OS.get_name()
# 	get_tree().paused = false
# 	pass 

# func _on_saving_time_timeout() -> void:
# 	get_tree().paused = false
# 	SaveManager.save_game()
# 	scene.change_scene_to_file("res://SaveAndLoad/LoadMenu.tscn")
# 	queue_redraw()
# 	queue_free()

# func _on_menu_pressed() -> void:
# 	%TouchControls.visible = false
# 	get_tree().paused = true
# 	%Panel.visible = true
# 	pass # Replace with function body.

# func _on_save_pressed() -> void:
# 	%TouchControls.visible = false
# 	%Panel.visible = false
# 	%CanvasLayer.visible = false
# 	%SavingTime.start()
# 	pass # Replace with function body.

# func _on_back_pressed() -> void:
# 	%TouchControls.visible = true
# 	%Panel.visible = false
# 	get_tree().paused = false
# 	pass # Replace with function body.

# func _on_add_player_pressed() -> void:
# 	multiplayerBridge.join_named_match(SaveManager.get_world_name())
# 	pass # Replace with function body.

# func _on_host_pressed() -> void:
# 	var k = randi_range(1, 65535)
# 	print("port: " + str(k))
# 	RoomInfo.port = 55555
# 	peer.create_server(5555)
# 	multiplayer.multiplayer_peer = peer
# 	multiplayer.peer_connected.connect(
# 	func(pid):
# 		print(pid)
# 		add_player(pid)
# 		multiplayer.get_unique_id()
# 		)
# 	multiplayer.peer_disconnected.connect(
# 		func(pid):
# 			print(pid)
# 			get_node(str(pid)).queue_free()
# 	)
# 	%World.broadcast()
# 	$Broadcaster.start()
# 	RoomInfo.name = SaveManager.get_world_name()
# 	pass # Replace with function body.
# var udp : PacketPeerUDP
# var listner: PacketPeerUDP
# @export var broadcastPort: int = 8912

# var RoomInfo = {"name":"name", "port": 0}
# func _on_broadcaster_timeout() -> void:
# 	var data = JSON.stringify(RoomInfo)
# 	var packet = data.to_ascii_buffer()
# 	%World.broadcaster_timeout(packet)
# 	pass # Replace with function body.

# func cleanUp():
# 	$Broadcaster.stop()
# 	if udp != null:
# 		udp.close()

# func _exit_tree():
# 	cleanUp()
	
# var webrtc_peer = PacketPeerUDP.new()

# # var multiplayerBridge : NakamaMultiplayerBridge

# # func setupMultiplayerBridge():
# # 	multiplayerBridge = NakamaMultiplayerBridge.new(socket)
# # 	multiplayerBridge.match_join_error.connect(onMatchJoinError)
# # 	var multiplayer = get_tree().get_multiplayer()
# # 	multiplayer.set_multiplayer_peer(multiplayerBridge.multiplayer_peer)
# # 	multiplayer.peer_connected.connect(onPeerConnected)
# # 	multiplayer.peer_disconnected.connect(onPeerDisconnected)
	

# func onMatchJoinError(error):
# 	print("Unable to join match: " + error.message)
# func onPeerConnected(id):
# 	print("Peer connected id is : " + str(id))
# func onPeerDisconnected(id):
# 	print("Peer disconnected id is : " + str(id))


# func _on_tester_pressed() -> void:
# 	session = await client.authenticate_email_async("test2@gmail.com", "password", "l")
# 	socket = Nakama.create_socket_from(client)
# 	await socket.connect_async(session)
# 	var account = await client.get_account_async(session)
# 	$player_id.text = account.user.id
# 	print(account)
	
# 	socket.connected.connect(conectedsocket)
# 	socket.closed.connect(onSocketClosed)
# 	socket.received_error.connect(onSocketReceivedError)
	
# 	socket.received_match_presence.connect(onMatchPresence)
# 	socket.received_match_state.connect(onMatchState)
# 	setupMultiplayerBridge()
	
# 	pass # Replace with function body.


# func _on_player_name_pressed() -> void:
# 	session = await client.authenticate_email_async("test@gmail.com", "password", "k")
# 	socket = Nakama.create_socket_from(client)
# 	await socket.connect_async(session)
# 	var account = await client.get_account_async(session)
# 	$player_id.text = account.user.id
# 	print(account)
	
# 	socket.connected.connect(conectedsocket)
# 	socket.closed.connect(onSocketClosed)
# 	socket.received_error.connect(onSocketReceivedError)
	
# 	socket.received_match_presence.connect(onMatchPresence)
# 	socket.received_match_state.connect(onMatchState)
# 	setupMultiplayerBridge()
	
# 	pass # Replace with function body.

# @rpc("any_peer")
# func sendData(message):
# 	print(message)

# func _on_send_pressed() -> void:
# 	sendData.rpc("hello")
# 	pass # Replace with function body.
