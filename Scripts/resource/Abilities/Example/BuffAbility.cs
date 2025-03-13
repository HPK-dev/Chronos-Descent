using ChronosDescent.Scripts.node;
using Godot;

namespace ChronosDescent.Scripts.resource.Abilities.Example;

[GlobalClass]
public partial class BuffAbility : Ability
{
    private double _lastTick;

    public BuffAbility()
    {
        Name = "Buff";
        Description = "Apply a beneficial effect";
        Type = AbilityType.Toggle; // Default to toggle
        Cooldown = 1.0; // Short cooldown for toggle abilities
    }

    [Export] public Effect BuffEffect { get; set; }
    [Export] public bool TargetSelf { get; set; } = true;
    [Export] public double Range { get; set; } = 150.0;
    [Export] public double AreaOfEffect { get; set; } // 0 means single target

    [Export] public double TickThreshold { get; set; } = 1.0;

    public override void Initialize()
    {
        BuffEffect.Duration = 1.1;
    }

    protected override void OnToggleOn()
    {
        if (BuffEffect == null) return;

        // Apply buff when toggled on
        if (TargetSelf) ApplyEffectToTarget(Caster);

        if (AreaOfEffect > 0) ApplyEffectToArea();

        GD.Print($"{Caster.Name} activated {Name}");
    }

    protected override void OnToggleOff()
    {
        if (BuffEffect == null) return;

        // Remove buff when toggled off
        if (TargetSelf) RemoveEffectFromTarget(Caster);

        if (AreaOfEffect > 0) RemoveEffectFromArea();

        GD.Print($"{Caster.Name} deactivated {Name}");
    }

    protected override void OnToggleTick(double delta)
    {
        // For toggle abilities, ensure the effect is maintained
        if (BuffEffect == null) return;

        _lastTick += delta;

        if (_lastTick < TickThreshold) return;

        _lastTick = 0;

        // Reapply the effect if needed
        if (TargetSelf && !Caster.HasEffect(BuffEffect.Name)) ApplyEffectToTarget(Caster);


        if (AreaOfEffect > 0)
            ApplyEffectToArea();
    }

    protected override void OnPassiveTick(double delta)
    {
        // For passive abilities, ensure the effect is always active
        if (BuffEffect == null) return;

        if (!Caster.HasEffect(BuffEffect.Name)) ApplyEffectToTarget(Caster);
    }

    private void ApplyEffectToTarget(Entity target)
    {
        if (BuffEffect != null) target.ApplyEffect(BuffEffect);
    }

    private void RemoveEffectFromTarget(Entity target)
    {
        if (BuffEffect != null) target.RemoveEffect(BuffEffect.Identifier);
    }

    private void ApplyEffectToArea()
    {
        var targets = Caster.GetTree().GetNodesInGroup("Entity");

        foreach (var node in targets)
        {
            if (!CheckEntityInRange(node)) continue;

            ApplyEffectToTarget((Entity)node);
        }
    }

    private void RemoveEffectFromArea()
    {
        var targets = Caster.GetTree().GetNodesInGroup("Entity");

        foreach (var node in targets)
        {
            if (!CheckEntityInRange(node)) continue;

            RemoveEffectFromTarget((Entity)node);
        }
    }


    private bool CheckEntityInRange(Node node)
    {
        return node is Entity target &&
               (target != Caster || TargetSelf) &&
               Caster.GlobalPosition.DistanceTo(target.GlobalPosition) <= Range;
    }
}