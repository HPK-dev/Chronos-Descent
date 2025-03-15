using Godot;

namespace ChronosDescent.Scripts.resource;

/// <summary>
/// Ability that charges up while holding the button and releases when letting go.
/// </summary>
[GlobalClass]
public partial class BaseChargedAbility : BaseAbility
{
    private bool _isCharging;

    [ExportGroup("Charged ability properties")]
    [Export] public new double MaxChargeTime { get; set; } = 1.0;
    [Export] public new bool AutoCastWhenFull { get; set; } = true;
    [Export] public new double MinChargeTime { get; set; } = 0.2;

    public new double CurrentChargeTime { get; protected set; }

    public new bool IsCharging
    {
        get => _isCharging;
        protected set
        {
            if (_isCharging == value) return;
            _isCharging = value;
            OnStateChanged(new AbilityStateEventArgs(this, 
                _isCharging ? AbilityState.Charging : AbilityState.Default));
        }
    }

    public override bool CanActivate()
    {
        return base.CanActivate() && !IsCharging;
    }

    public override void Activate()
    {
        if (!CanActivate()) return;

        // Start charging
        IsCharging = true;
        CurrentChargeTime = 0.0;

        GD.Print($"{Caster.Name} started charging {Name}");
    }

    public override void Update(double delta)
    {
        base.Update(delta);
        
        if (!IsCharging) return;
        
        CurrentChargeTime += delta;
        
        if (CurrentChargeTime >= MaxChargeTime && AutoCastWhenFull)
        {
            ReleaseCharge();
        }
    }

    /// <summary>
    /// Release a charged ability
    /// </summary>
    public new void ReleaseCharge()
    {
        if (!IsCharging) return;

        // Calculate charge percentage
        var chargePercentage = (CurrentChargeTime - MinChargeTime) / (MaxChargeTime - MinChargeTime);
        chargePercentage = Mathf.Clamp(chargePercentage, 0.0, 1.0);

        // Execute the ability with charge percentage
        ExecuteEffect(chargePercentage);

        // Reset charging state
        IsCharging = false;
        CurrentChargeTime = 0.0;

        // Start cooldown
        StartCooldown();

        GD.Print($"{Caster.Name} released {Name} with {chargePercentage * 100}% charge");
    }

    /// <summary>
    /// Cancel a charging ability without executing it
    /// </summary>
    public new void CancelCharge()
    {
        if (!IsCharging) return;

        // Reset charging state without executing
        IsCharging = false;
        CurrentChargeTime = 0.0;

        OnChargingCanceled();
    }

    /// <summary>
    /// Execute the ability effect with a power multiplier based on charge time
    /// </summary>
    protected new virtual void ExecuteEffect(double powerMultiplier)
    {
        // Base implementation does nothing
        // Override in derived classes
        GD.Print($"Executing ability {Name} with power {powerMultiplier}");
    }

    protected new virtual void OnChargingCanceled()
    {
        GD.Print($"{Name} charge canceled");
    }
}