using System.Collections.Generic;
using System.Linq;
using ChronosDescent.Scripts.node.Component;
using Godot;

namespace ChronosDescent.Scripts.resource.Abilities;

[GlobalClass]
public partial class TimeRewindAbility : BaseChanneledAbility
{
    private List<TimeManipulationComponent.PositionSpan> _positionHistory = [];
    private double _rewindProgress;

    public TimeRewindAbility()
    {
        Name = "Time Rewind";
        Description = "Rewind time to a previous position";
        Cooldown = 15.0;
        ChannelingDuration = 1.0;
    }

    [Export] public double RewindDuration { get; set; } = 3.0;


    public override bool CanActivate()
    {
        // Check if there's enough history to rewind
        if (Caster.TimeManipulation == null) return false;

        if (!base.CanActivate()) return false;

        _positionHistory = Caster.TimeManipulation.GetPositionHistory(RewindDuration).ToList();
        return _positionHistory.Count > 0;
    }

    public override void Activate()
    {
        base.Activate();

        // Start rewinding
        _rewindProgress = 0.0;

        // Disable movement control during rewind
        Caster.Moveable = false;

        // Pause recording during rewind
        Caster.TimeManipulation.PauseRecording();

        GD.Print($"{Caster.Name} activated {Name} - rewinding {RewindDuration} seconds");
    }


    // For channeled version, rewind continuously while channeling
    protected override void OnChannelingStart()
    {
        // Pause recording during channeling
        Caster.TimeManipulation.PauseRecording();

        GD.Print($"{Caster.Name} started channeling {Name}");
    }

    protected override void OnChannelingTick(double delta)
    {
        // Calculate what percentage of the rewind we've completed
        var progress = CurrentChannelingTime / ChannelingDuration;


        // Find the position to interpolate to
        var index = (int)(_positionHistory.Count * progress);
        if (index < _positionHistory.Count) Caster.Position = _positionHistory[index].Position;
    }

    protected override void OnChannelingComplete()
    {
        // Set final position
        if (_positionHistory.Count > 0) Caster.Position = _positionHistory[^1].Position;

        CleanUp();

        GD.Print($"{Caster.Name} completed channeling {Name}");
    }

    protected override void OnChannelingInterrupt()
    {
        CleanUp();

        GD.Print($"{Caster.Name}'s {Name} channeling was interrupted");
    }

    private void CleanUp()
    {
        // Resume time recording
        Caster.TimeManipulation.ResumeRecording();

        // Re-enable movement
        Caster.Moveable = true;

        // Clean up
        _positionHistory.Clear();

        // Start cooldown
        StartCooldown();
    }
}