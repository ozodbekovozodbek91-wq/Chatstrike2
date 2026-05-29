import React, { useState } from 'react';

interface Player {
  id: string;
  username: string;
  level: number;
  wins: number;
}

export const Dashboard: React.FC = () => {
  const [player, setPlayer] = useState<Player | null>(null);

  return (
    <div className="dashboard">
      <h1>Chatstrike2</h1>
      {player ? (
        <>
          <h2>{player.username}</h2>
          <p>Level: {player.level}</p>
          <p>Wins: {player.wins}</p>
        </>
      ) : (
        <p>Loading...</p>
      )}
      <button>Play</button>
    </div>
  );
};
