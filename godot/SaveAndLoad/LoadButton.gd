extends Button

var SaveName
var Date
var ImagePath
var SeedGame

signal LoadButtonDown(date, saveName, imagePath, seedGame)

func SetupButton(data):
	SaveName = data.name
	Date = Time.get_datetime_string_from_unix_time(data.dateTime)
	ImagePath = data.imgPath
	SeedGame = data.seed

func _on_button_down() -> void:
	LoadButtonDown.emit(Date, SaveName, ImagePath, SeedGame)
	pass # Replace with function body.
