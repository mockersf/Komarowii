extends Node2D



func draw_circle_custom(radius, maxerror = 0.25):

	if radius <= 0.0:
		return

	var maxpoints = 2048

	var points = PoolVector2Array([])

	for i in maxpoints:
		var phi = i * PI * 2.0 / maxpoints
		var v = Vector2(sin(phi), cos(phi))
		points.push_back(v * radius)

	draw_polyline(points, Color(1.0, 1.0, 1.0), 0.5, true)

func _ready():
	pass # Replace with function body.

func _draw():
	draw_circle_custom(5)
	pass
