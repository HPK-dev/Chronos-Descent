﻿using System;
using Godot;

namespace ChronosDescent.Scripts.node.Component;

[GlobalClass]
public partial class CombatComponent : Node
{
    private StatsComponent _stats;
    private AnimationComponent _animation;

    [Signal]
    public delegate void DeathEventHandler(Entity entity);

    public void Initialize(StatsComponent stats, AnimationComponent animation)
    {
        _stats = stats;
        _animation = animation;
    }

    public void TakeDamage(double amount)
    {
        if (_stats == null) return;


        _stats.UpdateStats(stats => stats.Health -= amount);
        _animation?.PlayAnimation("hurt");

        if (_stats.GetHealth() <= 0)
        {
            HandleDeath();
        }
    }

    public void Heal(double amount)
    {
        _stats?.UpdateStats(stats => stats.Health = Math.Min(stats.Health + amount, stats.MaxHealth));
    }

    public async void HandleDeath()
    {
        // Signal death event
        EmitSignal(SignalName.Death);

        // Play death animation
        _animation?.PlayAnimation("death");

        await ToSignal(GetTree().CreateTimer(0.5), SceneTreeTimer.SignalName.Timeout);

        // Queue free the parent entity
        GetParent().QueueFree();
    }
}