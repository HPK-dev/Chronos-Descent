using ChronosDescent.Scripts.node.Component;
using ChronosDescent.Scripts.UI;
using Godot;
using Effect = ChronosDescent.Scripts.resource.Effect;

namespace ChronosDescent.Scripts.node;

// Base Entity class that manages the components
[GlobalClass]
public partial class Entity : CharacterBody2D
{
    public delegate void EntityDeathEventHandler(Entity entity);

    public Vector2 AimDirection;

    public bool Moveable = true;
    private CollisionShape2D _collision;

    public event EntityDeathEventHandler EntityDeath;

    protected virtual void OnEntityDeath(Entity entity)
    {
        EntityDeath?.Invoke(entity);
    }

    
    public override void _Ready()
    {
        // Get component references
        Stats = GetNode<StatsComponent>("StatsComponent");
        EffectManager = GetNode<EffectManagerComponent>("EffectManagerComponent");
        DamageIndicatorManager = GetNode<DamageIndicatorManagerComponent>("DamageIndicatorManagerComponent");
        Combat = GetNode<CombatComponent>("CombatComponent");
        
        TimeManipulation = GetNodeOrNull<TimeManipulationComponent>("TimeManipulationComponent");
        AbilityManager = GetNodeOrNull<AbilityManagerComponent>("AbilityManagerComponent");
        Animation = GetNodeOrNull<AnimationComponent>("AnimationComponent");

        // Setup component connections
        EffectManager.Initialize(Stats);
        Combat?.Initialize(Stats, Animation);

        _collision = GetNode<CollisionShape2D>("CollisionShape2D");

        AddToGroup("Entity");
    }

    public override void _Process(double delta)
    {
    }

    public override void _PhysicsProcess(double delta)
    {
        // Movement logic would go here or in a separate MovementComponent
    }

    // Public methods that delegate to components
    public void TakeDamage(double amount, DamageIndicator.DamageType damageType = DamageIndicator.DamageType.Normal)
    {
        Combat.TakeDamage(amount, damageType);
    }

    public void Heal(double amount)
    {
        Combat.Heal(amount);
    }

    // Virtual method for derived classes to override
    public virtual async void OnEntityDeath()
    {
        // Signal death event
        OnEntityDeath(this);

        // Play death animation
        Animation?.PlayAnimation("death");

        await ToSignal(GetTree().CreateTimer(0.5), SceneTreeTimer.SignalName.Timeout);

        // Queue free the parent entity
        QueueFree();
    }
    
    // @formatter:off
    // Component references
    // Must have
    public StatsComponent Stats;
    public CombatComponent Combat;
    public DamageIndicatorManagerComponent DamageIndicatorManager;
    public EffectManagerComponent EffectManager;

    // Optional
    public TimeManipulationComponent TimeManipulation;
    public AnimationComponent Animation;
    public AbilityManagerComponent AbilityManager;
    // @formatter:on

    #region Effect

    public void ApplyEffect(Effect effect)
    {
        EffectManager.ApplyEffect(effect);
    }

    public void RemoveEffect(string effectId)
    {
        EffectManager.RemoveEffect(effectId);
    }

    public void RemoveAllEffects()
    {
        EffectManager.RemoveAllEffects();
    }

    public bool HasEffect(string effectId)
    {
        return EffectManager.HasEffect(effectId);
    }

    #endregion


    #region Ability

    public void ActivateAbility(AbilityManagerComponent.Slot slot)
    {
        if (AbilityManager == null)
        {
            Util.PrintWarning($"This entity({this}) has no ability manager!");
            return;
        }

        AbilityManager.ActivateAbility(slot);
    }

    public void ReleaseChargedAbility(AbilityManagerComponent.Slot slot)
    {
        if (AbilityManager == null)
        {
            Util.PrintWarning($"This entity({this}) has no ability manager!");
            return;
        }

        AbilityManager.ReleaseChargedAbility(slot);
    }

    public void CancelChargedAbility()
    {
        if (AbilityManager == null)
        {
            Util.PrintWarning($"This entity({this}) has no ability manager!");
            return;
        }

        AbilityManager.CancelChargedAbility();
    }

    public void InterruptChannelingAbility()
    {
        if (AbilityManager == null)
        {
            Util.PrintWarning($"This entity({this}) has no ability manager!");
            return;
        }

        AbilityManager.InterruptChannelingAbility();
    }

    #endregion


    public void DisableCollision(bool val)
    {
        _collision.Disabled = val;
    }
}