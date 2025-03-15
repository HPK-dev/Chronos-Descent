using Godot;

namespace ChronosDescent.Scripts.resource;

/// <summary>
/// Passive ability that is always active and doesn't require activation.
/// </summary>
[GlobalClass]
public partial class BasePassiveAbility : BaseAbility
{
    /// <summary>
    /// Passive abilities cannot be manually activated
    /// </summary>
    public override bool CanActivate()
    {
        return false;
    }

    /// <summary>
    /// Passive abilities are always active and don't need manual activation
    /// </summary>
    public override void Activate()
    {
        // Passive abilities can't be activated manually
        GD.Print($"{Name} is a passive ability and cannot be manually activated");
    }

    public override void Update(double delta)
    {
        base.Update(delta);

        // Continuously update passive effect
        OnPassiveTick(delta);
    }

    /// <summary>
    /// Called every frame to update the passive effect
    /// </summary>
    protected new virtual void OnPassiveTick(double delta)
    {
        // Override in derived classes to provide passive effects
    }
}