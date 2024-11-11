import * as proto from "@aurora-launcher/proto";
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';

import { ServerButton } from '../../components/ServerButton';
import SkinView from '../../components/SkinView';
import { useTitlebar } from '../../components/TitleBar/hooks';
import classes from './index.module.sass';

export default function ServersList() {
    const {
        hideTitlebarBackBtn,
        showTitlebarSettingsBtn,
        resetTitlebarTitleText,
        showTitlebarLogoutBtn,
    } = useTitlebar();

    const [servers, setServers] = useState<proto.Server[]>([]);
    const navigate = useNavigate();

    useEffect(() => {
        hideTitlebarBackBtn();
        showTitlebarLogoutBtn();
        showTitlebarSettingsBtn();
        resetTitlebarTitleText();
        invoke('get_servers').then((res: proto.ServersResponse) => {
            setServers(res.servers);
        });
        //launcherAPI.rpc.updateActivity('default');
    }, []);

    const selectServer = async (server: proto.Server) => {
        console.log(server);
        //await launcherAPI.scenes.serversList.selectServer(server);
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
