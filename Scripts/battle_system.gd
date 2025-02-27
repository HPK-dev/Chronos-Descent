extends BattleSystem


func _ready() -> void:
	Console.add_command("entities", _cmd_print_entities)
	Console.add_command("components", _cmd_get_components, 1)
	Console.add_command("kill", _cmd_kill_entity, 1)
	Console.add_command("spawn", _cmd_spawn_entity, ["name", "x", "y"], 1)
	
func _exit_tree() -> void:
	Console.remove_command("entities")
	Console.remove_command("components")
	Console.remove_command("kill")
	Console.remove_command("spawn")
	


func _handle_result(result: Dictionary) -> void:
	if not result:
		return
	
	var message :String = result.get("message", "<EOF>") 
	var is_error :bool = result.get("is_error", false)
	
	if not is_error:
		Console.print_line(message)
	else :
		Console.print_error(message)
		
		

func _cmd_print_entities () -> void:
	_handle_result(cmd_print_entities())
	
func _cmd_get_components(arg:String) -> void:
	_handle_result(cmd_get_components(arg))
	
func _cmd_kill_entity(arg:String) -> void:
	_handle_result(cmd_kill_entity(arg))
	
func _cmd_spawn_entity(n:String, x:String, y:String) -> void:
	_handle_result(cmd_spawn_entity(n,x,y))
	

# =================================================================

func cmd_kill_entity(instance_id: String) -> Dictionary:
	var gd_entity
	
	if instance_id.is_valid_int():
		gd_entity = instance_from_id(instance_id.to_int())
	else:
		return {"message": "Failed to parse instance ID", "is_error": true}

	# Check if we got a valid entity
	if gd_entity != null:
		gd_entity._on_entity_died()
		return {"message": "Killed entity: " + str(gd_entity), "is_error": false}
	else:
		return {"message": "Invalid entity instance", "is_error": true}
