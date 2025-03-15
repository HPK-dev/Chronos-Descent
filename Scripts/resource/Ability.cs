// using System;
// using ChronosDescent.Scripts.node;
// using Godot;
//
// namespace ChronosDescent.Scripts.resource;
//
// [GlobalClass]
// public partial class Ability : Resource
// {
//     public enum AbilityState
//     {
//         Default = 0,
//         Charging = 1,
//         Channeling = 2,
//         ToggledOn = 3,
//         ToggledOff = 4,
//         Cooldown = 5
//     }
//
//     // Ability type
//     public enum AbilityType
//     {
//         Active, // Standard one-time use ability
//         Passive, // Always active ability
//         Toggle, // Can be turned on/off
//         Charged, // Hold to charge up, release to activate
//         Channeled // Continuous effect while channeling
//     }
//
//     private double _currentCooldown;
//
//     private bool _isChanneling;
//
//     private bool _isCharging;
//
//     // Toggle state for Toggle abilities
//     private bool _isToggled;
//
//     // Entity that owns this ability
//     public Entity Caster;
//
//     [ExportGroup("Metadata")]
//     // Basic ability properties
//     [Export]
//     public string Name { get; set; } = "Ability";
//
//     [Export] public string Description { get; set; } = "";
//     [Export] public Texture2D Icon { get; set; }
//
//     // Cooldown and cost properties
//     [Export] public double Cooldown { get; set; } = 5.0; // In seconds
//
//     public double CurrentCooldown
//     {
//         get => _currentCooldown;
//         private set
//         {
//             if (!(Math.Abs(_currentCooldown - value) > 0.001)) return;
//             _currentCooldown = value;
//             OnCooldownChanged(new AbilityCooldownEventArgs(this));
//         }
//     }
//
//     [Export] public AbilityType Type { get; set; } = AbilityType.Active;
//
//     public bool IsToggled
//     {
//         get => _isToggled;
//         protected set
//         {
//             if (_isToggled == value) return;
//             _isToggled = value;
//             OnStateChanged(new AbilityStateEventArgs(this,
//                 _isToggled ? AbilityState.ToggledOn : AbilityState.ToggledOff));
//         }
//     }
//
//     [ExportGroup("Charged ability properties")]
//     // Charging properties for Charged abilities
//     [Export]
//     public double MaxChargeTime { get; set; } = 1.0;
//
//     [Export] public bool AutoCastWhenFull { get; set; } = true;
//     [Export] public double MinChargeTime { get; set; } = 0.2;
//
//     public double CurrentChargeTime { get; protected set; }
//
//     public bool IsCharging
//     {
//         get => _isCharging;
//         protected set
//         {
//             if (_isCharging == value) return;
//             _isCharging = value;
//             OnStateChanged(new AbilityStateEventArgs(this, _isCharging ? AbilityState.Charging : AbilityState.Default));
//         }
//     }
//
//     // Channeling properties for Channeled abilities
//     [ExportGroup("Channeled ability properties")]
//     [Export]
//     public double ChannelingDuration { get; set; } = 3.0;
//
//     public double CurrentChannelingTime { get; protected set; }
//
//     public bool IsChanneling
//     {
//         get => _isChanneling;
//         protected set
//         {
//             if (_isChanneling == value) return;
//             _isChanneling = value;
//             OnStateChanged(new AbilityStateEventArgs(this,
//                 _isChanneling ? AbilityState.Channeling : AbilityState.Default));
//         }
//     }
//
//     // Whether the ability is on cooldown
//     public bool IsOnCooldown => CurrentCooldown > 0.0;
//
//     // C# events instead of Godot signals
//     public event EventHandler<AbilityStateEventArgs> StateChanged;
//     public event EventHandler<AbilityCooldownEventArgs> CooldownChanged;
//
//     public virtual void Initialize()
//     {
//     }
//
//     // Whether the ability can be activated
//     public virtual bool CanActivate()
//     {
//         // Check cooldown
//         if (IsOnCooldown) return false;
//
//         return Type switch
//         {
//             // For toggle abilities, we can always toggle
//             AbilityType.Toggle => true,
//             // For charged abilities, we can start charging
//             AbilityType.Charged => !IsCharging,
//             // For channeled abilities, we can start channeling
//             AbilityType.Channeled => !IsChanneling,
//             _ => true
//         };
//     }
//
//     // Activate the ability
//     public virtual void Activate()
//     {
//         // AbilityType.Active:
//         //     ExecuteEffect();
//         //     StartCooldown();
//
//
//         // AbilityType.Passive:
//         //     // Passive abilities are always active
//
//
//         // AbilityType.Toggle:
//         //    IsToggled = !IsToggled;
//         //    if (IsToggled)
//         //        OnToggleOn();
//         //    else
//         //        OnToggleOff();
//
//
//         // AbilityType.Charged
//         //     IsCharging = true;
//         //     CurrentChargeTime = 0.0;
//
//
//         // AbilityType.Channeled
//         //      IsChanneling = true;
//         //      CurrentChannelingTime = 0.0;
//         //      OnChannelingStart();
//     }
//
//     protected void UpdateCooldown(double delta)
//     {
//         if (CurrentCooldown <= 0) return;
//
//         CurrentCooldown -= delta;
//         if (CurrentCooldown < 0) CurrentCooldown = 0;
//     }
//
//
//     public virtual void Update(double delta)
//     {
//         UpdateCooldown(delta);
//     }
//
//
//     // Release a charged ability
//     public void ReleaseCharge()
//     {
//         if (!IsCharging) return;
//
//         // Calculate charge percentage
//         var chargePercentage = (CurrentChargeTime - MinChargeTime) / (MaxChargeTime - MinChargeTime);
//         chargePercentage = Mathf.Clamp(chargePercentage, 0.0, 1.0);
//
//         // Execute the ability with charge percentage
//         ExecuteEffect(chargePercentage);
//
//         // Reset charging state
//         IsCharging = false;
//         CurrentChargeTime = 0.0;
//
//         // Start cooldown
//         StartCooldown();
//
//         GD.Print($"{Caster.Name} released {Name} with {chargePercentage * 100}% charge");
//     }
//
//     // Cancel a charging ability
//     public void CancelCharge()
//     {
//         if (!IsCharging) return;
//
//         // Reset charging state without executing
//         IsCharging = false;
//         CurrentChargeTime = 0.0;
//
//         OnChargingCanceled();
//     }
//
//     // Complete a channeled ability normally
//     public void CompleteChanneling()
//     {
//         if (!IsChanneling) return;
//
//         // Execute final effect
//         OnChannelingComplete();
//
//         // Reset channeling state
//         IsChanneling = false;
//         CurrentChannelingTime = 0.0;
//
//         // Start cooldown
//         StartCooldown();
//     }
//
//     // Interrupt a channeled ability
//     public void InterruptChanneling()
//     {
//         if (!IsChanneling) return;
//
//         // Execute interrupt effect
//         OnChannelingInterrupt();
//
//         // Reset channeling state
//         IsChanneling = false;
//         CurrentChannelingTime = 0.0;
//
//         // Start a potentially reduced cooldown (50% by default)
//         StartCooldown(0.5);
//     }
//
//     // Start the ability cooldown
//     protected virtual void StartCooldown(double multiplier = 1.0)
//     {
//         CurrentCooldown = Cooldown * multiplier;
//     }
//
//     // Execute the ability effect - to be overridden by derived classes
//     protected virtual void ExecuteEffect()
//     {
//         // Base implementation does nothing
//         // Override in derived classes
//         GD.Print($"Executing ability {Name}");
//     }
//
//     protected virtual void ExecuteEffect(double powerMultiplier)
//     {
//         // Base implementation does nothing
//         // Override in derived classes
//         GD.Print($"Executing ability {Name} with power {powerMultiplier}");
//     }
//
//     // Toggle ability callbacks
//     protected virtual void OnToggleOn()
//     {
//         GD.Print($"Ability {Name} toggled ON");
//     }
//
//     protected virtual void OnToggleOff()
//     {
//         GD.Print($"Ability {Name} toggled OFF");
//     }
//
//     protected virtual void OnToggleTick(double delta)
//     {
//     }
//
//     // Passive ability callback
//     protected virtual void OnPassiveTick(double delta)
//     {
//     }
//
//     // Channeling ability callbacks
//     protected virtual void OnChannelingStart()
//     {
//         GD.Print($"Started channeling {Name}");
//     }
//
//     protected virtual void OnChannelingTick(double delta)
//     {
//     }
//
//     protected virtual void OnChannelingComplete()
//     {
//         GD.Print($"Completed channeling {Name}");
//     }
//
//     protected virtual void OnChannelingInterrupt()
//     {
//         GD.Print($"Channeling of {Name} interrupted");
//     }
//
//     protected virtual void OnChargingCanceled()
//     {
//     }
//
//     // Event invokers
//     protected virtual void OnStateChanged(AbilityStateEventArgs e)
//     {
//         StateChanged?.Invoke(this, e);
//     }
//
//     protected virtual void OnCooldownChanged(AbilityCooldownEventArgs e)
//     {
//         CooldownChanged?.Invoke(this, e);
//     }
//
//     // Custom event args
//     public class AbilityStateEventArgs(Ability ability, AbilityState state) : EventArgs
//     {
//         public Ability Ability { get; } = ability;
//         public AbilityState State { get; } = state;
//     }
//
//     public class AbilityCooldownEventArgs(Ability ability) : EventArgs
//     {
//         public Ability Ability { get; } = ability;
//     }
// }