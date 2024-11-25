import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export function usePingServer(server?: Server) {
    const [players, setPlayers] = useState({ online: 0, max: 0 });

    useEffect(() => {
        if (!server) return;

        invoke('ping', {
            host: server.server_info.ip,
            port: server.server_info.port,
        }).then((message) => {
            setPlayers({ online: (message as Ping).online || 0, max: (message as Ping).max || 0 });
        });
    }, [server]);

    return players;
}
