using Godot;

namespace ChronosDescent.Scripts.resource;

/// <summary>
/// Standard one-time use ability that executes immediately and goes on cooldown.
/// </summary>
[GlobalClass]
public partial class BaseActiveAbility : BaseAbility
{
    public override void Activate()
    {
        if (!CanActivate()) return;

        // Execute effect immediately
        ExecuteEffect();
        
        // Start cooldown
        StartCooldown();
        
        // Notify state change
        OnStateChanged(new AbilityStateEventArgs(this, AbilityState.Cooldown));
    }
}