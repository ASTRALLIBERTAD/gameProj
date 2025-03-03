extends Node

var udp := PacketPeerUDP.new()
@export var serverInfo: PackedScene
signal joinGame(ip)

# Listener for incoming packets
var listener: PacketPeerUDP
@export var listenPort: int = 8912



func _ready():
	

	setUp()




func setUp():
	listener = PacketPeerUDP.new()
	var ok = listener.bind(listenPort)
	if ok == OK:
		print("Successfully bound to port:", listenPort)
	else:
		print("Failed to bind to port:", listenPort)


var stun = PacketPeerUDP.new()

func _process(_delta):
	while listener.get_available_packet_count() > 0:
		var serverip = listener.get_packet_ip()
		var serverport = listener.get_packet_port()
		var packet = listener.get_packet()
		
		# Convert bytes to string using get_string_from_utf8() instead of get_string_from_ascii()
		var data = packet.get_string_from_ascii()
		
		# Add error handling for JSON parsing
		var json = JSON.new()
		var parse_result = json.parse(data)
		
		if parse_result == OK:
			var roomInfo = json.get_data()
			print("Server IP: " + serverip + " Server Port: " + str(serverport) + " Room info: " + str(roomInfo))
			
			# Check if room already exists
			var room_exists = false
			for i in $CanvasLayer/Panel/VBoxContainer.get_children():
				if i.name == roomInfo.name:
					i.get_node("Ip").text = serverip
					i.get_node("PlayerCount").text = roomInfo.port
					return
			
			# Only create new room if it doesn't exist
			if !room_exists:
				var currentInfo = serverInfo.instantiate()
				currentInfo.name = roomInfo.name
				
				currentInfo.get_node("Ip").text = str(serverip)
				currentInfo.get_node("PlayerCount").text = roomInfo.port
				currentInfo.get_node("Name").text = roomInfo.name
				$CanvasLayer/Panel/VBoxContainer.add_child(currentInfo)
				# Connect the signal using lambda to pass the IP
				currentInfo.joinGame.connect(joinbyIp)

func joinbyIp(ip):
	joinGame.emit(ip)




func d(ip):
	var peer = ENetMultiplayerPeer.new()
	var error = peer.create_client(ip, 55555)
	
	if error == OK:
		multiplayer.multiplayer_peer = peer
		print("Connecting ", ip ) 
		get_tree().change_scene_to_file("res://World.tscn")
	else:
		print("Failed to create client. Error code:", error)

func _on_back_pressed():
	get_tree().change_scene_to_file("res://SaveAndLoad/LoadMenu.tscn")

func _exit_tree():
	cleanUp()

func cleanUp():
	listener.close()
