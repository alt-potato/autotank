extends Area2D
signal hit

@export var move_speed = 200 # px/s
@export var turn_speed = deg_to_rad(90) # rad/s

var screen_size

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	screen_size = get_viewport_rect().size


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	var dr = 0
	var dtheta = 0
	
	if Input.is_action_pressed("move_forward"):
		dr += 1
	if Input.is_action_pressed("move_backward"):
		dr -= 1
	if Input.is_action_pressed("turn_left"):
		dtheta -= 1
	if Input.is_action_pressed("turn_right"):
		dtheta += 1
	
	var radius = dr * move_speed * delta
	var angle = rotation + dtheta * turn_speed * delta
	
	var velocity = Vector2(radius, 0).rotated(angle)
	position += velocity
	position = position.clamp(Vector2.ZERO, screen_size)
	rotation = angle


func _on_body_entered(_body: Node2D) -> void:
	hide()
	hit.emit()
	# tell the physics engine to disable collisions
	$CollisionShape2D.set_deferred("disabled", true) 


func start(pos):
	position = pos
	show()
	$CollisionShape2D.disabled = false
