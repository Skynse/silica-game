extends Node

var sim: GDWorld
func _ready():
	sim = GDWorld.new()
	var dat: PackedByteArray = sim.get_data()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):

	pass
