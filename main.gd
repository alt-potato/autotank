extends Node

@export var bullet_scene: PackedScene

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	new_game()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	pass


func game_over() -> void:
	$StartTimer.stop()
	$TestBulletTimer.stop()

func new_game():
	$Player.start($PlayerStartPosition.position)
	$StartTimer.start()


func _on_start_timer_timeout() -> void:
	$TestBulletTimer.start()


func _on_test_bullet_timer_timeout() -> void:
	# Create a new instance of the Mob scene.
	var bullet = bullet_scene.instantiate()

	# Choose a random location on Path2D.
	var bullet_spawn_location = $BulletSpawnPath/BulletSpawnPosition
	bullet_spawn_location.progress_ratio = randf()

	# Set the mob's position to the random location.
	bullet.position = bullet_spawn_location.position

	# Set the mob's direction perpendicular to the path direction.
	var direction = bullet_spawn_location.rotation + PI / 2

	# Add some randomness to the direction.
	direction += randf_range(-PI / 4, PI / 4)
	bullet.rotation = direction

	# Choose the velocity for the mob.
	var velocity = Vector2(randf_range(150.0, 250.0), 0.0)
	bullet.linear_velocity = velocity.rotated(direction)

	# Spawn the mob by adding it to the Main scene.
	add_child(bullet)
