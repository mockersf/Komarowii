extends Node2D

var rng = RandomNumberGenerator.new()
var death_probability= 0.00001
var change_probability = 0.05
var change_percent = 0.1

func _ready():
    rng.randomize()

func _process(delta):
	var proba= rng.randf()
	# should this star die
	if proba < death_probability:
		queue_free()
	# should this star change brightness
	if proba < change_probability:
		var color = get_node("Star").color
		if rng.randf() < 0.5:
			color = color.lightened(change_percent)
		else:
			color = color.darkened(change_percent)
		get_node("Star").color = color

func _on_VisibilityNotifier2D_screen_exited():
	queue_free()
