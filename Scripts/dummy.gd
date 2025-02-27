extends Entity


func _on_entity_died() -> void:
	var instance_id = get_instance_id()
	var battle_system = get_node("/root/BattleScene/BattleSystem")
	battle_system.unregister_entity(instance_id)
	queue_free()
