extends BattleSystem

func _ready() -> void:
	Console.add_command("entities", _cmd_print_entities)
	Console.add_command("components", _cmd_get_components, 1)
	
func _exit_tree() -> void:
	Console.remove_command("entities")
	Console.remove_command("components")


func _cmd_print_entities () -> void:
	Console.print_line(cmd_print_entities())
	
func _cmd_get_components(arg:String) -> void:
	Console.print_line(cmd_get_components(arg))
