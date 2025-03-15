using Godot;
using System;

[GlobalClass]
public partial class ItemsCollectibles : Resource
{
    [Export] public string Name = "";
    [Export] public Texture2D Icon;
}
