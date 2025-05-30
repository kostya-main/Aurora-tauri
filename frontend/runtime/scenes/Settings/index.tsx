import { useEffect, useState } from 'react';

import { getVersion } from '@tauri-apps/api/app';
import { revealItemInDir, openUrl } from '@tauri-apps/plugin-opener';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import logo from '../../assets/images/logo.png';
import If from '../../components/If';
import { MemoryRange } from '../../components/MemoryRange';
import { useTitlebar } from '../../components/TitleBar/hooks';
import classes from './index.module.sass';
import { getTotalMemory, store } from '../../../utils';

export default function Settings() {
    const {
        showTitlebarBackBtn,
        setTitlebarTitleText,
        hideTitlebarSettingsBtn,
        hideTitlebarLogoutBtn,
    } = useTitlebar();

    useEffect(() => {
        hideTitlebarLogoutBtn();
        showTitlebarBackBtn();
        hideTitlebarSettingsBtn();
        setTitlebarTitleText('Настройки лаунчера');
        
        getVersion().then((ver) => SetVersion(ver));
        getCurrentWindow().title().then((title) => SetTitle(title));
        getTotalMemory().then((mem) => SetTotalMemory(mem));
        store.get('dir').then((dir) => SetDir(dir));
        store.get('autoConnect').then((autoConnect) => SetAutoConnect(autoConnect));
        store.get('fullScreen').then((fullScreen) => SetfullScreen(fullScreen));
        store.get('useMemory').then((useMemory) => SetMemory(useMemory));
        store.get('startDebug').then((startDebug) => SetStartDebug(startDebug));
    }, []);

    const [main, EditButtonMain] = useState(true);
    const [info, EditButtonInfo] = useState(false);

    const [totalMemory, SetTotalMemory] = useState(0);
    const [dir, SetDir] = useState('');
    const [autoConnect, SetAutoConnect] = useState(false);
    const [fullScreen, SetfullScreen] = useState(false);
    const [memory, SetMemory] = useState(0);
    const [startDebug, SetStartDebug] = useState(false);
    const [version, SetVersion] = useState('v0.0.0');
    const [title, SetTitle] = useState('Launcher');

    const Button = (type: string) => {
        switch (type) {
            case 'main':
                EditButtonMain(true);
                EditButtonInfo(false);
                return;
            case 'info':
                EditButtonMain(false);
                EditButtonInfo(true);
                return;
        }
    };

    return (
        <div className={classes.window}>
            <div className={classes.buttonsList}>
                <div className={classes.buttons}>
                    <button onClick={() => Button('main')} disabled={main}>
                        Основное
                    </button>
                    <button onClick={() => Button('info')} disabled={info}>
                        О лаунчере
                    </button>
                </div>
            </div>
            <If state={main}>
                <div className={classes.options}>
                    <label className={classes.checkbox}>
                        <input
                            type="checkbox"
                            checked={fullScreen}
                            onChange={(e) => {
                                store.set('fullScreen', Boolean(e.target.checked))
                                SetfullScreen(Boolean(e.target.checked))
                            }
                            }
                        />
                        <span className={classes.checkboxSwitch}></span>
                        Запуск игры во весь экран
                    </label>
                    <label className={classes.checkbox}>
                        <input
                            type="checkbox"
                            checked={startDebug}
                            onChange={(e) =>{
                                store.set('startDebug', Boolean(e.target.checked))
                                SetStartDebug(Boolean(e.target.checked))
                            }
                            }
                        />
                        <span className={classes.checkboxSwitch}></span>
                        Запуск игры в дебаг режиме
                    </label>
                    <label className={classes.checkbox}>
                        <input
                            type="checkbox"
                            checked={autoConnect}
                            onChange={(e) =>{
                                store.set('autoConnect', Boolean(e.target.checked))
                                SetAutoConnect(Boolean(e.target.checked))
                            }
                            }
                        />
                        <span className={classes.checkboxSwitch}></span>
                        Автоматический вход на сервер
                    </label>
                    <label>
                        Выделено оперативной памяти: {memory}MB
                    </label>
                    <br />
                    <MemoryRange
                        limit={totalMemory}
                        onChange={(e) =>{
                            store.set('useMemory', Number(e.target.value))
                            SetMemory(Number(e.target.value))
                        }
                        }
                        value={memory}
                    />
                    <label>
                    Расположение игры
                    </label>
                    <br />
                    <div className={classes.changeDir}>
                        <button className={classes.openDir} onClick={() => revealItemInDir(dir)}>
                        {dir}
                        </button>
                        <button className={classes.editDir} onClick={() => {
                            openDialog({
                                title: 'Выберите директорию для игры',
                                defaultPath: dir,
                                directory: true,
                            }).then((res) => {
                                if (res) {
                                    store.set('dir', res);
                                    SetDir(res);
                                }
                            });
                        }}>
                            Смена директории
                        </button>
                    </div>
                </div>
            </If>
            <If state={info}>
                <div className={classes.options}>
                    <div className={classes.logo}>
                        <img src={logo} alt="Aurora Launcher" />
                    </div>
                    <div className={classes.launcherName}>
                        <center>
                            <h1>{title}</h1>
                        </center>
                    </div>
                    <div className={classes.icons}>
                        <button
                            onClick={() =>
                                openUrl(
                                    'https://www.youtube.com/@AuroraTeamRu',
                                )
                            }
                        >
                            <svg width="35" height="35" viewBox="0 0 32 32">
                                <path
                                    d="M29.398 5.928c-0.867-0.624-3.283-1.453-13.406-1.453-10.848 0-12.458 0.952-12.975 1.264-2.749 1.652-3.010 8.963-3.034 10.445 0.033 1.958 0.349 8.473 3.032 10.084 0.515 0.311 2.116 1.257 12.976 1.257 10.13 0 12.543-0.825 13.406-1.447 2.343-1.686 2.612-7.49 2.619-9.92-0.006-1.954-0.211-8.496-2.619-10.23zM28.23 24.456c-0.349 0.251-2.35 1.069-12.239 1.069-9.56 0-11.566-0.742-11.945-0.971-0.965-0.579-1.984-3.652-2.063-8.376 0.079-4.713 1.111-8.153 2.066-8.726 0.38-0.229 2.393-0.977 11.943-0.977 9.881 0 11.887 0.824 12.239 1.076 0.857 0.617 1.773 3.865 1.787 8.613-0.014 4.823-0.936 7.678-1.787 8.291zM22.513 15.213l-8.998-5.522c-0.309-0.187-0.695-0.191-1.008-0.014-0.315 0.178-0.509 0.511-0.509 0.871v11.045c0 0.36 0.194 0.694 0.509 0.871 0.152 0.086 0.322 0.129 0.491 0.129 0.178 0 0.358-0.048 0.516-0.143l8.998-5.524c0.3-0.181 0.483-0.506 0.483-0.856s-0.183-0.676-0.483-0.857zM13.999 19.822v-7.503l6.063 3.751z"
                                    fill="white"
                                />
                            </svg>
                        </button>
                        <button
                            onClick={() =>
                                openUrl(
                                    'https://discord.gg/2NvYTcv',
                                )
                            }
                        >
                            <svg width="35" height="35" viewBox="0 0 24 24">
                                <path
                                    d="M18.8943 4.34399C17.5183 3.71467 16.057 3.256 14.5317 3C14.3396 3.33067 14.1263 3.77866 13.977 4.13067C12.3546 3.89599 10.7439 3.89599 9.14391 4.13067C8.99457 3.77866 8.77056 3.33067 8.58922 3C7.05325 3.256 5.59191 3.71467 4.22552 4.34399C1.46286 8.41865 0.716188 12.3973 1.08952 16.3226C2.92418 17.6559 4.69486 18.4666 6.4346 19C6.86126 18.424 7.24527 17.8053 7.57594 17.1546C6.9466 16.92 6.34927 16.632 5.77327 16.2906C5.9226 16.184 6.07194 16.0667 6.21061 15.9493C9.68793 17.5387 13.4543 17.5387 16.889 15.9493C17.0383 16.0667 17.177 16.184 17.3263 16.2906C16.7503 16.632 16.153 16.92 15.5236 17.1546C15.8543 17.8053 16.2383 18.424 16.665 19C18.4036 18.4666 20.185 17.6559 22.01 16.3226C22.4687 11.7787 21.2836 7.83202 18.8943 4.34399ZM8.05593 13.9013C7.01058 13.9013 6.15725 12.952 6.15725 11.7893C6.15725 10.6267 6.98925 9.67731 8.05593 9.67731C9.11191 9.67731 9.97588 10.6267 9.95454 11.7893C9.95454 12.952 9.11191 13.9013 8.05593 13.9013ZM15.065 13.9013C14.0196 13.9013 13.1652 12.952 13.1652 11.7893C13.1652 10.6267 13.9983 9.67731 15.065 9.67731C16.121 9.67731 16.985 10.6267 16.9636 11.7893C16.9636 12.952 16.1317 13.9013 15.065 13.9013Z"
                                    fill="white"
                                />
                            </svg>
                        </button>
                        <button
                            onClick={() =>
                                openUrl(
                                    'https://aurora-launcher.ru/',
                                )
                            }
                        >
                            <svg width="35" height="35" viewBox="0 0 24 24">
                                <path
                                    d="M7.8 12L7.05 12L7.8 12ZM16.2 12H16.95H16.2ZM12 16.2V16.95V16.2ZM14.1729 22.2749L14.3273 23.0088L14.1729 22.2749ZM9.82712 22.2749L9.67269 23.0088L9.82712 22.2749ZM2.27554 8.03225L1.58122 7.74867H1.58122L2.27554 8.03225ZM1.7251 14.1729L0.991173 14.3273L1.7251 14.1729ZM9.82712 1.7251L9.67269 0.991173L9.82712 1.7251ZM14.1729 1.7251L14.3273 0.991174L14.1729 1.7251ZM21.6399 8.07014L21.8576 8.78785L21.6399 8.07014ZM2.35887 8.06976L2.14116 8.78747L2.35887 8.06976ZM21.0312 8.3185C21.4944 9.45344 21.75 10.6959 21.75 12H23.25C23.25 10.4983 22.9553 9.06352 22.42 7.75174L21.0312 8.3185ZM21.75 12C21.75 12.6927 21.6779 13.3678 21.541 14.0184L23.0088 14.3273C23.167 13.5757 23.25 12.7972 23.25 12H21.75ZM21.541 14.0184C20.7489 17.7827 17.7828 20.7489 14.0184 21.541L14.3273 23.0088C18.6735 22.0943 22.0943 18.6735 23.0088 14.3273L21.541 14.0184ZM14.0184 21.541C13.3678 21.6779 12.6927 21.75 12 21.75V23.25C12.7972 23.25 13.5757 23.167 14.3273 23.0088L14.0184 21.541ZM12 21.75C11.3072 21.75 10.6322 21.6779 9.98156 21.541L9.67269 23.0088C10.4242 23.167 11.2028 23.25 12 23.25V21.75ZM2.25 12C2.25 10.6949 2.50601 9.45149 2.96986 8.31584L1.58122 7.74867C1.0451 9.06127 0.75 10.4971 0.75 12H2.25ZM9.98156 21.541C6.21724 20.7489 3.25112 17.7827 2.45903 14.0184L0.991173 14.3273C1.90571 18.6735 5.32647 22.0943 9.67269 23.0088L9.98156 21.541ZM2.45903 14.0184C2.32213 13.3678 2.25 12.6927 2.25 12H0.75C0.75 12.7972 0.83303 13.5757 0.991173 14.3273L2.45903 14.0184ZM2.96986 8.31584C4.17707 5.36016 6.79381 3.1298 9.98155 2.45903L9.67269 0.991173C5.99032 1.76602 2.97369 4.33941 1.58122 7.74867L2.96986 8.31584ZM9.98155 2.45903C10.6322 2.32213 11.3072 2.25 12 2.25V0.75C11.2028 0.75 10.4242 0.83303 9.67269 0.991173L9.98155 2.45903ZM12 2.25C12.6927 2.25 13.3678 2.32213 14.0184 2.45903L14.3273 0.991174C13.5757 0.833031 12.7972 0.75 12 0.75V2.25ZM14.0184 2.45903C17.2071 3.13 19.8245 5.3615 21.0312 8.3185L22.42 7.75174C21.0281 4.34096 18.0108 1.76625 14.3273 0.991174L14.0184 2.45903ZM13.4584 1.95309C13.7482 2.8614 14.8215 6.35621 15.2615 9.5682L16.7476 9.36461C16.289 6.01664 15.1813 2.41835 14.8874 1.49712L13.4584 1.95309ZM15.2615 9.5682C15.3795 10.4292 15.45 11.2568 15.45 12L16.95 12C16.95 11.1681 16.8715 10.269 16.7476 9.36461L15.2615 9.5682ZM21.4222 7.35242C20.2692 7.70212 18.1033 8.3164 15.8685 8.72886L16.1407 10.204C18.4546 9.7769 20.6809 9.14473 21.8576 8.78785L21.4222 7.35242ZM15.8685 8.72886C14.5129 8.97904 13.1579 9.15 12 9.15L12 10.65C13.2874 10.65 14.743 10.4619 16.1407 10.204L15.8685 8.72886ZM15.45 12C15.45 13.1009 15.2954 14.3808 15.0647 15.671L16.5413 15.935C16.7797 14.6019 16.95 13.2252 16.95 12L15.45 12ZM15.0647 15.671C14.5591 18.4992 13.7097 21.2593 13.4584 22.0469L14.8874 22.5029C15.145 21.6956 16.0181 18.8613 16.5413 15.935L15.0647 15.671ZM22.0469 13.4584C21.2593 13.7097 18.4992 14.5591 15.671 15.0647L15.935 16.5413C18.8613 16.0181 21.6956 15.145 22.5029 14.8874L22.0469 13.4584ZM15.671 15.0647C14.3808 15.2954 13.1009 15.45 12 15.45L12 16.95C13.2252 16.95 14.6019 16.7797 15.935 16.5413L15.671 15.0647ZM12 15.45C10.8991 15.45 9.61923 15.2954 8.32897 15.0647L8.06496 16.5413C9.39807 16.7797 10.7748 16.95 12 16.95L12 15.45ZM8.32897 15.0647C5.50076 14.5591 2.74066 13.7097 1.95309 13.4584L1.49712 14.8874C2.30437 15.145 5.13873 16.0181 8.06496 16.5413L8.32897 15.0647ZM7.05 12C7.05 13.2252 7.22032 14.6019 7.45867 15.935L8.93526 15.671C8.70456 14.3808 8.55 13.1009 8.55 12L7.05 12ZM7.45867 15.935C7.98188 18.8613 8.85504 21.6956 9.11261 22.5029L10.5416 22.0469C10.2903 21.2593 9.44094 18.4992 8.93526 15.671L7.45867 15.935ZM9.11261 1.49712C8.81867 2.41835 7.711 6.01664 7.25235 9.36461L8.73846 9.5682C9.17849 6.35621 10.2518 2.8614 10.5416 1.95309L9.11261 1.49712ZM7.25235 9.36461C7.12846 10.269 7.05 11.1681 7.05 12L8.55 12C8.55 11.2568 8.62052 10.4292 8.73846 9.5682L7.25235 9.36461ZM12 9.15C10.8421 9.15 9.4871 8.97904 8.13152 8.72886L7.85929 10.204C9.25697 10.4619 10.7126 10.65 12 10.65L12 9.15ZM8.13152 8.72886C5.89586 8.31625 3.72921 7.70168 2.57657 7.35205L2.14116 8.78747C3.3175 9.14428 5.54457 9.77675 7.85929 10.204L8.13152 8.72886ZM21.38 7.3695C21.3919 7.3633 21.4065 7.35719 21.4222 7.35242L21.8576 8.78785C21.933 8.76498 22.0039 8.73569 22.0712 8.70074L21.38 7.3695ZM1.88425 8.67209C1.96322 8.72038 2.04888 8.75948 2.14116 8.78747L2.57657 7.35205C2.60983 7.36214 2.64048 7.3763 2.66683 7.39242L1.88425 8.67209Z"
                                    fill="white"
                                />
                            </svg>
                        </button>
                        <button
                            onClick={() =>
                                openUrl(
                                    'https://github.com/AuroraTeam/AuroraLauncher',
                                )
                            }
                        >
                            <svg width="35" height="35" viewBox="0 0 48 48">
                                <path
                                    d="M24,2.5a21.5,21.5,0,0,0-6.8,41.9c1.08.2,1.47-.46,1.47-1s0-1.86,0-3.65c-6,1.3-7.24-2.88-7.24-2.88A5.7,5.7,0,0,0,9,33.68c-1.95-1.33.15-1.31.15-1.31a4.52,4.52,0,0,1,3.29,2.22c1.92,3.29,5,2.34,6.26,1.79a4.61,4.61,0,0,1,1.37-2.88c-4.78-.54-9.8-2.38-9.8-10.62a8.29,8.29,0,0,1,2.22-5.77,7.68,7.68,0,0,1,.21-5.69s1.8-.58,5.91,2.2a20.46,20.46,0,0,1,10.76,0c4.11-2.78,5.91-2.2,5.91-2.2a7.74,7.74,0,0,1,.21,5.69,8.28,8.28,0,0,1,2.21,5.77c0,8.26-5,10.07-9.81,10.61a5.12,5.12,0,0,1,1.46,4c0,2.87,0,5.19,0,5.9s.39,1.24,1.48,1A21.5,21.5,0,0,0,24,2.5"
                                    fill="white"
                                />
                            </svg>
                        </button>
                    </div>
                    <div className={classes.version}>
                        <h5>Версия лаунчера: {version}</h5>
                    </div>
                </div>
            </If>
        </div>
    );
}
