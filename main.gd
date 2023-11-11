extends Node

var sim: GDWorld
func _ready():
	sim = GDWorld.new()
	
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if sim.world.running:
		sim.world.tick()
	pass
