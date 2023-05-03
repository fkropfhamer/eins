using System.Diagnostics;

namespace backend.Models;

public class Game
{
    private Timer? _loopTimer;
    private readonly List<Player> _players = new();
    private bool _isStarted = false;

    public void Start()
    {
        if (_isStarted)
        {
            return;
        }
        
        _loopTimer = new Timer(
            GameLoop, null, 2000, 2000
        );
    }

    public void Stop()
    {
        _loopTimer?.Dispose();
    }

    private void GameLoop(object? state)
    {
        if (_players.Count == 0)
        {
            Stop();
        }
        
        foreach (var player in _players)
        {
            player.Send("update");
        }
    }

    public void AddPlayer(Player player)
    {
        _players.Add(player);
    }
}