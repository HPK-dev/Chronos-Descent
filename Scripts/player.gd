extends Entity

func _ready() -> void:
	get_parent().register_entity(self)
