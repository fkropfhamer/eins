using System.Diagnostics;
using backend.Manager;
using Microsoft.AspNetCore.SignalR;

namespace backend.Hubs;

public class GameHub : Hub
{
    private static readonly GameManager GameManager = new();
    public void SendMessage(string message)
    {
        var player = GameManager.GetPlayer(Context.ConnectionId);
        if (player == null)
        {
            return;
        }

        var username = player.Username;
        
        Clients.All.SendAsync("ReceiveMessage", username, message);
    }

    public Task Echo(string message)
    {
        return Clients.Client(Context.ConnectionId)
            .SendAsync("send", message);
    }

    public void JoinGame(Guid gameId) 
    {
        GameManager.JoinGame(Context.ConnectionId, gameId);
    }

    public void StartGame()
    {
        GameManager.StartGame(Context.ConnectionId);
    }

    public void CreateGame()
    {
        GameManager.CreateGame(Context.ConnectionId);
    }

    public void Join(string username)
    {
        var connectionId = Context.ConnectionId;
        var client = Clients.Client(connectionId);

        GameManager.CreatePlayer(connectionId, client, username);
    }

    public override Task OnDisconnectedAsync(Exception? exception)
    {
        var connectionId = Context.ConnectionId;
        GameManager.RemovePlayer(connectionId);

        return base.OnDisconnectedAsync(exception);
    }
}