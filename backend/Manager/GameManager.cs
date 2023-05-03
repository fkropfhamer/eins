using System.Collections.Concurrent;
using System.Diagnostics;
using backend.Models;
using Microsoft.AspNetCore.SignalR;

namespace backend.Manager;

public class GameManager
{
    private readonly ConcurrentDictionary<Guid, Game> _games = new ();
    private readonly ConcurrentDictionary<string, Player> _players = new ();
    
    public Player? GetPlayer(string connectionId)
    {
        return !_players.TryGetValue(connectionId, out var player) ? null : player;
    }
    
    private Game? GetGame(Guid gameId)
    {
        return !_games.TryGetValue(gameId, out var game) ? null : game;
    }

    public Player CreatePlayer(string connectionId, ISingleClientProxy client, string username)
    {
        var player = new Player(client, username);

        _players.TryAdd(connectionId, player);

        return player;
    }

    public void CreateGame(string connectionId)
    {
        var player = GetPlayer(connectionId);
        if (player == null)
        {
            return;
        }
        
        var game = new Game();
        var id = Guid.NewGuid();

        _games.TryAdd(id, game);
        game.AddPlayer(player);
        player.Game = game;
        player.SendGameCreated(id);
    }

    public void StartGame(string connectionId)
    {
        var player = GetPlayer(connectionId);
        
        Debug.WriteLine("starting");

        player?.Game?.Start();
    }

    public void RemovePlayer(string connectionId)
    {
        _players.TryRemove(connectionId, out _);
    }

    public void JoinGame(string connectionId, Guid gameId)
    {
        var player = GetPlayer(connectionId);
        if (player == null)
        {
            return;
        }

        
        var game = GetGame(gameId);
        if (game == null)
        {
            return;
        }
        
        game.AddPlayer(player);
        player.Game = game;
        player.SendJoinedGame(gameId);
    }
}