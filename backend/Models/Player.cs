using Microsoft.AspNetCore.SignalR;

namespace backend.Models;

public class Player
{
    private readonly IClientProxy _client;
    public Game? Game { get; set; }
    public string Username { get; }
    
    public Player(IClientProxy client, string username)
    {
        _client = client;
        Username = username;
    }
    
    public void Send(string message)
    {
        _client.SendAsync("Message", message);
    }

    public void SendGameCreated(Guid gameId)
    {
        _client.SendAsync("GameCreated", gameId);
    }

    public void SendJoinedGame(Guid gameId)
    {
        _client.SendAsync("JoinedGame", gameId);
    }
}