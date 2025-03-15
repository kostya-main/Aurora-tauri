import { useEffect, useState } from 'react';
import { navigate } from "wouter-preact/use-browser-location";
import { invoke } from '@tauri-apps/api/core';

import { ServerButton } from '../../components/ServerButton';
import SkinView from '../../components/SkinView';
import { useTitlebar } from '../../components/TitleBar/hooks';
import { useSelectServer } from '../../hooks/selectServer';
import classes from './index.module.sass';

export default function ServersList() {
    const {
        hideTitlebarBackBtn,
        showTitlebarSettingsBtn,
        resetTitlebarTitleText,
        showTitlebarLogoutBtn,
    } = useTitlebar();

    const [servers, setServers] = useState<Server[]>([]);
    const { setSelectServerState } = useSelectServer();

    useEffect(() => {
        hideTitlebarBackBtn();
        showTitlebarLogoutBtn();
        showTitlebarSettingsBtn();
        resetTitlebarTitleText();
        invoke('get_servers').then((message) => {
            setServers((message as Get_servers).servers);
        });
        invoke('set_activity', {status: "default"});
    }, []);

    const selectServer = async (server: Server) => {
        setSelectServerState(server);
        navigate('/ServerPanel');
    };

    return (
        <div className={classes.window}>
            <div className={classes.skinView}>
                <SkinView />
            </div>
            <div className={classes.serverList}>
                {servers.map((server, i) => (
                    <ServerButton
                        key={i}
                        server={server}
                        onClick={() => selectServer(server)}
                    />
                ))}
            </div>
        </div>
    );
}
