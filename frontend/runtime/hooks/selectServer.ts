import { atom, useSetAtom } from 'jotai';


export const stateServer = atom(
    defaultServer()
);

export function useSelectServer() {
    const setSelectServerState = useSetAtom(stateServer);
    return {
        setSelectServerState,
    };
}

function defaultServer() {
    const server: Server = {
        profile_uuid: '',
        server_info: {
            title: '',
            hostname: null,
            ip: '',
            port: 0,
        },
    };
    return server;
}