using System;
using System.Collections.Generic;
using ChronosDescent.Scripts.resource;
using Godot;

namespace ChronosDescent.Scripts.node.Component;

[GlobalClass]
public partial class AbilityManagerComponent : Node
{
    public enum Slot
    {
        NormalAttack = 0,
        Primary = 1,
        Secondary = 2,
        WeaponUlt = 3,
        Unknown = 8
    }

    // List of abilities - sized to match enum values
    private readonly BaseAbility[] _abilities = new BaseAbility[4];

    private readonly Dictionary<BaseAbility, EventHandler<BaseAbility.AbilityCooldownEventArgs>>
        _cooldownChangedHandlers =
            new();

    // Dictionary to store references to event handlers for unsubscribing
    private readonly Dictionary<BaseAbility, EventHandler<BaseAbility.AbilityStateEventArgs>> _stateChangedHandlers =
        new();

    // Track currently active ability slot
    private Slot _currentActiveSlot = Slot.Unknown;

    // C# events instead of Godot signals
    public event EventHandler<AbilityEventArgs> AbilityActivated;
    public event EventHandler<AbilityCooldownEventArgs> AbilityCooldownChanged;
    public event EventHandler<AbilityStateEventArgs> AbilityStateChanged;
    public event EventHandler<AbilitySlotEventArgs> AbilityChanged;

    public override void _Ready()
    {
    }

    public override void _PhysicsProcess(double delta)
    {
        UpdateAbilities(delta);
    }

    private void UpdateAbilities(double delta)
    {
        foreach (var ability in _abilities) ability?.Update(delta);
    }

    // Add an ability
    public void SetAbility(Slot slot, BaseAbility ability)
    {
        if (slot == Slot.Unknown || ability == null) return;

        // If slot already has an ability, clean it up first
        var existingAbility = GetAbility(slot);
        if (existingAbility != null)
        {
            UnsubscribeFromAbilityEvents(existingAbility);
            existingAbility.Caster = null;
        }

        var caster = (Entity)Owner;

        ability.Caster = caster;
        ability.Initialize();
        _abilities[(int)slot] = ability;

        // Subscribe to ability events
        SubscribeToAbilityEvents(ability, slot);

        OnAbilityChanged(new AbilitySlotEventArgs(ability, slot));
        GD.Print($"Added ability {ability.Name} to {caster.Name}");
    }

    // Remove an ability
    public void RemoveAbility(Slot slot)
    {
        if (slot == Slot.Unknown) return;

        var index = (int)slot;
        var ability = _abilities[index];

        if (ability == null) return;

        UnsubscribeFromAbilityEvents(ability);
        ability.Caster = null;
        _abilities[index] = null;

        // Clear active slot if removing active ability
        if (_currentActiveSlot == slot) _currentActiveSlot = Slot.Unknown;

        OnAbilityChanged(new AbilitySlotEventArgs(null, slot));
        GD.Print($"Removed ability {ability.Name} from slot {slot}");
    }

    // Subscribe to ability events
    private void SubscribeToAbilityEvents(BaseAbility ability, Slot slot)
    {
        // Create handler for state changed
        EventHandler<BaseAbility.AbilityStateEventArgs> stateHandler = (sender, e) =>
            HandleAbilityStateChanged(ability, slot);

        // Create handler for cooldown changed
        EventHandler<BaseAbility.AbilityCooldownEventArgs> cooldownHandler = (sender, e) =>
            HandleAbilityCooldownChanged(ability);

        // Store handlers for later unsubscription
        _stateChangedHandlers[ability] = stateHandler;
        _cooldownChangedHandlers[ability] = cooldownHandler;

        // Subscribe to events
        ability.StateChanged += stateHandler;
        ability.CooldownChanged += cooldownHandler;
    }

    // Unsubscribe from ability events
    private void UnsubscribeFromAbilityEvents(BaseAbility ability)
    {
        if (_stateChangedHandlers.TryGetValue(ability, out var stateHandler))
        {
            ability.StateChanged -= stateHandler;
            _stateChangedHandlers.Remove(ability);
        }

        if (_cooldownChangedHandlers.TryGetValue(ability, out var cooldownHandler))
        {
            ability.CooldownChanged -= cooldownHandler;
            _cooldownChangedHandlers.Remove(ability);
        }
    }

    // Handle ability state changed
    private void HandleAbilityStateChanged(BaseAbility ability, Slot slot)
    {
        var state = BaseAbility.AbilityState.Default;

        if (ability.IsOnCooldown)
        {
            state = BaseAbility.AbilityState.Cooldown;
        }
        else if (ability is BaseChargedAbility { IsCharging: true })
        {
            state = BaseAbility.AbilityState.Charging;
            _currentActiveSlot = slot;
        }
        else if (ability is BaseChanneledAbility { IsChanneling: true })
        {
            state = BaseAbility.AbilityState.Channeling;
            _currentActiveSlot = slot;
        }
        else if (ability.IsToggled)
        {
            state = BaseAbility.AbilityState.ToggledOn;
        }
        else if (ability.Type == BaseAbility.AbilityType.Toggle && !ability.IsToggled)
        {
            state = BaseAbility.AbilityState.ToggledOff;
        }
        else if (!ability.IsCharging && !ability.IsChanneling)
        {
            // If ability was active and is no longer active, clear the slot
            if (GetSlotForAbility(ability) == _currentActiveSlot) _currentActiveSlot = Slot.Unknown;
        }

        OnAbilityStateChanged(new AbilityStateEventArgs(ability, state));
    }

    // Handle ability cooldown changed
    private void HandleAbilityCooldownChanged(BaseAbility ability)
    {
        OnAbilityCooldownChanged(new AbilityCooldownEventArgs(ability, ability.CurrentCooldown));

        switch (ability.CurrentCooldown)
        {
            // Also emit state changed if cooldown becomes 0
            case <= 0:
                OnAbilityStateChanged(new AbilityStateEventArgs(ability, BaseAbility.AbilityState.Default));
                break;
            case > 0 when !ability.IsCharging && !ability.IsChanneling && !ability.IsToggled:
                OnAbilityStateChanged(new AbilityStateEventArgs(ability, BaseAbility.AbilityState.Cooldown));
                break;
        }
    }

    // Get an ability by slot
    public BaseAbility GetAbility(Slot slot)
    {
        if (slot == Slot.Unknown || (int)slot >= _abilities.Length) return null;
        return _abilities[(int)slot];
    }

    // Get the slot for a specific ability
    private Slot GetSlotForAbility(BaseAbility ability)
    {
        for (var i = 0; i < _abilities.Length; i++)
            if (_abilities[i] == ability)
                return (Slot)i;

        return Slot.Unknown;
    }

    // Activate an ability by slot
    public void ActivateAbility(Slot slot)
    {
        if (slot == Slot.Unknown) return;

        var ability = GetAbility(slot);
        if (ability == null)
        {
            GD.Print($"Ability slot {slot} is empty");
            return;
        }

        // Don't allow activating a new channeled/charged ability while another is active
        if (ability is BaseChanneledAbility or BaseChargedAbility &&
            _currentActiveSlot != Slot.Unknown && _currentActiveSlot != slot)
        {
            if (!IsAbilityOnCooldown(_currentActiveSlot))
            {
                GD.Print($"Cannot activate {ability.Name} while another ability is active");
                return;
            }

            _currentActiveSlot = Slot.Unknown;
        }

        if (!ability.CanActivate())
        {
            GD.Print($"Cannot activate {ability.Name}");
            return;
        }

        ability.Activate();
        OnAbilityActivated(new AbilityEventArgs(ability));

        switch (ability.Type)
        {
            // For active abilities, immediately emit cooldown state
            case BaseActiveAbility:
                OnAbilityStateChanged(new AbilityStateEventArgs(ability, BaseAbility.AbilityState.Cooldown));
                break;
            case BaseChanneledAbility or BaseChargedAbility:
                ((Entity)Owner).Moveable = false;
                break;
        }

        GD.Print($"Activated ability {ability.Name}");
    }

    // Release a charged ability
    public void ReleaseChargedAbility(Slot slot)
    {
        if (slot != _currentActiveSlot && slot != Slot.Unknown)
            // Only process if the requested slot is the active one
            return;

        if (_currentActiveSlot == Slot.Unknown)
        {
            Util.PrintWarning("Attempting to release a unknown charged ability");
            return;
        }

        var ability = GetAbility(_currentActiveSlot);
        if (ability?.IsCharging != true) return;

        ability.ReleaseCharge();
        _currentActiveSlot = Slot.Unknown;

        ((Entity)Owner).Moveable = true;

        GD.Print($"Released charge of {ability.Name}");
    }

    // Cancel a charging ability
    public void CancelChargedAbility()
    {
        if (_currentActiveSlot != Slot.Unknown)
        {
            var ability = GetAbility(_currentActiveSlot);
            if (ability?.IsCharging == true)
            {
                ability.CancelCharge();
                _currentActiveSlot = Slot.Unknown;
            }
            else
            {
                Util.PrintWarning("Current ability is not charging");
            }
        }
        else
        {
            Util.PrintWarning("No ability is charging");
        }
    }

    // Interrupt a channeling ability
    public void InterruptChannelingAbility()
    {
        if (_currentActiveSlot == Slot.Unknown)
        {
            Util.PrintWarning("Attempting to interrupt a unknown channeling ability");
            return;
        }

        var ability = GetAbility(_currentActiveSlot);
        if (ability?.IsChanneling != true) return;

        ability.InterruptChanneling();
        _currentActiveSlot = Slot.Unknown;

        GD.Print($"Interrupted channeling of {ability.Name}");
    }

    // Custom event args
    public class AbilityEventArgs : EventArgs
    {
        public AbilityEventArgs(BaseAbility ability)
        {
            Ability = ability;
        }

        public BaseAbility Ability { get; }
    }

    public class AbilityCooldownEventArgs : EventArgs
    {
        public AbilityCooldownEventArgs(BaseAbility ability, double cooldown)
        {
            Ability = ability;
            Cooldown = cooldown;
        }

        public BaseAbility Ability { get; }
        public double Cooldown { get; }
    }

    public class AbilityStateEventArgs : EventArgs
    {
        public AbilityStateEventArgs(BaseAbility ability, BaseAbility.AbilityState state)
        {
            Ability = ability;
            State = state;
        }

        public BaseAbility Ability { get; }
        public BaseAbility.AbilityState State { get; }
    }

    public class AbilitySlotEventArgs : EventArgs
    {
        public AbilitySlotEventArgs(BaseAbility ability, Slot slot)
        {
            Ability = ability;
            SlotValue = slot;
        }

        public BaseAbility Ability { get; }
        public Slot SlotValue { get; }
    }

    #region Event Invokers

    protected void OnAbilityActivated(AbilityEventArgs e)
    {
        AbilityActivated?.Invoke(this, e);
    }

    protected void OnAbilityCooldownChanged(AbilityCooldownEventArgs e)
    {
        AbilityCooldownChanged?.Invoke(this, e);
    }

    protected void OnAbilityStateChanged(AbilityStateEventArgs e)
    {
        AbilityStateChanged?.Invoke(this, e);
    }

    protected void OnAbilityChanged(AbilitySlotEventArgs e)
    {
        AbilityChanged?.Invoke(this, e);
    }

    #endregion

    #region BaseAbility State Helpers

    // Get the cooldown of an ability
    public double GetAbilityCooldown(Slot slot)
    {
        return GetAbility(slot)?.CurrentCooldown ?? 0.0;
    }

    // Get the cooldown percentage of an ability
    public double GetAbilityCooldownPercentage(Slot slot)
    {
        var ability = GetAbility(slot);
        if (ability == null || ability.Cooldown <= 0.0) return 0.0;

        return ability.CurrentCooldown / ability.Cooldown;
    }

    public bool IsAbilityReady(Slot slot)
    {
        return GetAbility(slot)?.CanActivate() ?? false;
    }

    public bool IsAbilityCharging(Slot slot)
    {
        var ability = GetAbility(slot);
        if (ability is BaseChargedAbility chargedAbility)
        {
            return charged BaseAbility.IsCharging;
        }

        return false;
    }

    public bool IsAbilityChanneling(Slot slot)
    {
        return GetAbility(slot)?.IsChanneling ?? false;
    }

    public bool IsAbilityOnCooldown(Slot slot)
    {
        return GetAbility(slot)?.IsOnCooldown ?? false;
    }

    public bool IsAbilityToggled(Slot slot)
    {
        return GetAbility(slot)?.IsToggled ?? false;
    }

    public Slot GetCurrentActiveSlot()
    {
        return _currentActiveSlot;
    }

    public bool HasActiveAbility()
    {
        return _currentActiveSlot != Slot.Unknown;
    }

    #endregion
}