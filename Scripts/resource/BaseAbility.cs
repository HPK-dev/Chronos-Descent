using System;
using ChronosDescent.Scripts.node;
using Godot;

namespace ChronosDescent.Scripts.resource;

/// <summary>
/// Base abstract class for all abilities in the game.
/// </summary>
[GlobalClass]
public abstract partial class BaseAbility : Resource
{
    public enum AbilityState
    {
        Default = 0,
        Charging = 1,
        Channeling = 2,
        ToggledOn = 3,
        ToggledOff = 4,
        Cooldown = 5
    }


    private double _currentCooldown;

    // Entity that owns this ability
    public Entity Caster;

    [ExportGroup("Metadata")] [Export] public string Name { get; set; } = "Ability";
    [Export] public string Description { get; set; } = "";
    [Export] public Texture2D Icon { get; set; }

    // Cooldown properties
    [Export] public double Cooldown { get; set; } = 5.0; // In seconds

    public double CurrentCooldown
    {
        get => _currentCooldown;
        protected set
        {
            if (!(Math.Abs(_currentCooldown - value) > 0.001)) return;
            _currentCooldown = value;
            OnCooldownChanged(new AbilityCooldownEventArgs(this));
        }
    }


    // Whether the ability is on cooldown
    public bool IsOnCooldown => CurrentCooldown > 0.0;

    // C# events
    public event EventHandler<AbilityStateEventArgs> StateChanged;
    public event EventHandler<AbilityCooldownEventArgs> CooldownChanged;

    public virtual void Initialize()
    {
    }

    /// <summary>
    /// Determines if the ability can be activated.
    /// </summary>
    public virtual bool CanActivate()
    {
        return !IsOnCooldown;
    }

    /// <summary>
    /// Activates the ability. This method should be overridden by derived classes.
    /// </summary>
    public abstract void Activate();

    /// <summary>
    /// Updates the ability state. This method should be called every frame.
    /// </summary>
    public virtual void Update(double delta)
    {
        UpdateCooldown(delta);
    }

    /// <summary>
    /// Updates the cooldown timer.
    /// </summary>
    protected void UpdateCooldown(double delta)
    {
        if (CurrentCooldown <= 0) return;

        CurrentCooldown -= delta;
        if (CurrentCooldown < 0) CurrentCooldown = 0;
    }

    /// <summary>
    /// Starts the ability cooldown.
    /// </summary>
    protected virtual void StartCooldown(double multiplier = 1.0)
    {
        CurrentCooldown = Cooldown * multiplier;
    }

    /// <summary>
    /// Executes the ability effect. This method should be overridden by derived classes.
    /// </summary>
    protected virtual void ExecuteEffect()
    {
        GD.Print($"Executing ability {Name}");
    }

    // Event invokers
    protected virtual void OnStateChanged(AbilityStateEventArgs e)
    {
        StateChanged?.Invoke(this, e);
    }

    protected virtual void OnCooldownChanged(AbilityCooldownEventArgs e)
    {
        CooldownChanged?.Invoke(this, e);
    }

    // Custom event args
    public class AbilityStateEventArgs(BaseAbility ability, AbilityState state) : EventArgs
    {
        public BaseAbility Ability { get; } = ability;
        public AbilityState State { get; } = state;
    }

    public class AbilityCooldownEventArgs(BaseAbility ability) : EventArgs
    {
        public BaseAbility Ability { get; } = ability;
    }
}