import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ServerButton } from '../../components/ServerButton';
import SkinView from '../../components/SkinView';
import classes from './index.module.sass';
import { useTitlebar } from '../../components/TitleBar/hooks';

export default function ServersList() {
    const { hideTitlebarBackBtn } = useTitlebar();
    hideTitlebarBackBtn();

    const [servers, setServers] = useState<[]>([]);
    const navigate = useNavigate();

    useEffect(() => {
        launcherAPI.scenes.serversList.getServers().then(setServers);
    }, []);

    const selectServer = async (server: string) => {
        await launcherAPI.scenes.serversList.selectServer(server);
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
