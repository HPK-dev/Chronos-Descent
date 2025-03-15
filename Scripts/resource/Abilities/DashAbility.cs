using System.Collections.Generic;
using ChronosDescent.Scripts.node;
using ChronosDescent.Scripts.node.Component;
using Godot;

namespace ChronosDescent.Scripts.resource.Abilities;

[GlobalClass]
public partial class DashAbility : BaseActiveAbility
{
    private Vector2 _dashDirection = Vector2.Zero;
    private Vector2 _dashTarget = Vector2.Zero;
    private bool _isDashing;

    // Add a HashSet to track damaged entities during this dash
    private HashSet<Entity> _damagedEntities = [];

    public DashAbility()
    {
        Name = "Dash";
        Description = "Quickly dash in a direction";
        Cooldown = 3.0;
    }

    [Export] public double DashDistance { get; set; } = 200.0;
    [Export] public double DashSpeed { get; set; } = 1000.0;
    [Export] public bool DamageOnDash { get; set; } = true;
    [Export] public double DashDamage { get; set; } = 15.0;
    [Export] public double DamageRadius { get; set; } = 50.0;

    protected override void ExecuteEffect()
    {
        // Get the direction to dash
        Vector2 direction;
        if (Caster is Player player)
        {
            // For player, use the aim direction
            direction = UserInputManager.Instance.AimInput;
            if (direction == Vector2.Zero)
                direction = Vector2.Right * (player.GetNode<AnimationComponent>("AnimationComponent").FlipH ? -1 : 1);
        }
        else
        {
            // For other entities, use velocity or default direction
            direction = Caster.Velocity.Normalized();
            if (direction == Vector2.Zero) direction = Vector2.Right;
        }

        // Set up the dash
        _isDashing = true;
        _dashDirection = direction.Normalized();
        _dashTarget = Caster.Position + _dashDirection * (float)DashDistance;

        // Clear the damaged entities set for this new dash
        _damagedEntities.Clear();

        // Disable collision
        Caster.DisableCollision(true);
        Caster.Moveable = false;

        GD.Print($"{Caster.Name} activated {Name} in direction {_dashDirection}");
    }

    public override void Update(double delta)
    {
        base.Update(delta);

        if (!_isDashing) return;

        // Calculate movement distance this frame
        var moveDistance = DashSpeed * delta;

        // Calculate the direction to target
        var currentPosition = Caster.Position;
        var distanceToTarget = currentPosition.DistanceTo(_dashTarget);

        if (distanceToTarget <= moveDistance)
        {
            // We've reached the target
            Caster.Position = _dashTarget;
            FinishDash();
        }
        else
        {
            // Move towards the target
            Caster.Position += _dashDirection * (float)moveDistance;

            // Check for damage on dash if enabled
            if (DamageOnDash) DealDashDamage();
        }
    }

    private void FinishDash()
    {
        if (Caster == null) return;

        // Re-enable movement
        Caster.Moveable = true;

        // Clean up
        _isDashing = false;

        // Enable collision
        Caster.DisableCollision(false);

        Caster.Moveable = true;
    }

    private void DealDashDamage()
    {
        // Find entities in dash damage radius
        var targets = Caster.GetTree().GetNodesInGroup("Entity");

        foreach (var node in targets)
        {
            if (node is not Entity target || target == Caster) continue;

            // Skip if this entity was already damaged during this dash
            if (_damagedEntities.Contains(target)) continue;

            var distance = Caster.GlobalPosition.DistanceTo(target.GlobalPosition);

            if (distance > DamageRadius) continue;
            
            target.TakeDamage(DashDamage);
            // Add to our set of damaged entities
            _damagedEntities.Add(target);
        }
    }
}