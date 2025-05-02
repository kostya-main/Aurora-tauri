/// <reference types="vite/client" />

interface Get_servers {
    servers: Server[];
}

interface Ping {
    online: number;
    max: number;
}

interface Auth {
    username: string;
    user_uuid: string;
    access_token: string;
    token: string;
    is_alex: boolean | null;
    skin_url: string | null;
    cape_url: string | null;
}

interface Server {
    profile_uuid: string;
    server_info: {
        title: string;
        hostname: string | null;
        ip: string;
        port: number;
    }
}