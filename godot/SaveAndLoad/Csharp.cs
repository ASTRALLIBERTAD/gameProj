using Godot;
using System;

public partial class Csharp : Control
{
	[Export]
	public PackedScene LoadButton;
	

	public Node SaveManager { get; set; }
	public static String BASE_PATH = "";
	private readonly Callable _on_button_down;

	public override void _Ready()
	{
		base._Ready();
		if (base.GetTree().Root.HasNode("/root/main"))
		{
			GetTree().Root.GetNode("/root/main").QueueFree();
			

		}
		SavedGamesGodot();
		
	}

	public string GetDir()
	{
		
		if (OS.GetName() == "Windows")
		{
			BASE_PATH = "user://";	
		}
		else if (OS.GetName() == "Android")
		{
			BASE_PATH = "/storage/emulated/0/Android/data/com.example.proj/files/";
		}
		return BASE_PATH;
	}

	public void SavedGamesGodot(){
		var dir = DirAccess.GetDirectoriesAt(GetDir() + "games");
		GD.Print(GetDir());

		foreach (var i in dir)
		{
			var button = LoadButton.Instantiate<Button>();
			if (button.HasSignal("LoadButtonDown")){
				if (button.Connect("LoadButtonDown", _on_button_down) == Error.Ok)
				{
					GD.Print("connected");
				}
				GD.Print("gettt");
			}
		}

	}
}
