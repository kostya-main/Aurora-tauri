import { useState, useEffect } from 'react';

export function usePingServer(server: string) {
    const [players, setPlayers] = useState({ online: 0, max: 0 });

    useEffect(() => {
        if (!server) {
            return;
        }

        fetch(
            `https://mcapi.us/server/status?ip=${server}&port=${server || 25565}`,
        )
            .then((res) => res.json())
            .then((res) => {
                if (!res.online) return;
                const { max, now } = res.players;
                setPlayers({ max, online: now });
            });
    }, [server]);

    return players;
}
